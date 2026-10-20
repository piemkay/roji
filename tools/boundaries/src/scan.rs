//! Finds I/O in Rust source: paths into the `std` modules that reach files, the network,
//! processes, the environment, threads or the clock; console macros; `extern` blocks; extern
//! crates other than `alloc` and `core`; and `include!` or `#[path]` reaching outside `src/`.
//! Also reads the attributes of a crate root.

use std::collections::BTreeSet;
use std::path::{Component, Path};
use std::str::FromStr;

use proc_macro2::{Spacing, Span, TokenStream, TokenTree};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

/// `std` paths an I/O-free crate must not use. A path matches when it equals one of these or
/// continues it (`std::fs::read` matches `std::fs`).
pub const IO_PATHS: &[&str] = &[
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::os",
    "std::path",
    "std::process",
    "std::thread",
    "std::time::Instant",
    "std::time::SystemTime",
];

/// Macros that write to the console.
pub const IO_MACROS: &[&str] = &["dbg", "eprint", "eprintln", "print", "println"];

/// One piece of I/O found in a source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    pub line: usize,
    pub message: String,
}

/// The crate-level attributes the boundary checks look for in a crate root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CrateAttributes {
    /// `#![forbid(unsafe_code)]`
    pub forbids_unsafe_code: bool,
    /// `#![no_std]`, unconditionally (not inside `cfg_attr`)
    pub no_std: bool,
}

/// Reads the inner attributes of a crate root. A file that does not parse has none.
pub fn crate_attributes(source: &str) -> CrateAttributes {
    let mut found = CrateAttributes::default();
    let Ok(file) = syn::parse_file(source) else {
        return found;
    };
    for attr in file
        .attrs
        .iter()
        .filter(|attr| matches!(attr.style, syn::AttrStyle::Inner(_)))
    {
        match &attr.meta {
            syn::Meta::Path(path) if path.is_ident("no_std") => found.no_std = true,
            syn::Meta::List(list) if list.path.is_ident("forbid") => {
                let lints = list.tokens.to_string();
                if lints.split(',').any(|lint| lint.trim() == "unsafe_code") {
                    found.forbids_unsafe_code = true;
                }
            }
            _ => {}
        }
    }
    found
}

/// Every piece of I/O in `source`, one finding per line and kind.
pub fn scan_source(source: &str) -> Vec<Finding> {
    let mut findings = BTreeSet::new();
    match syn::parse_file(source) {
        Ok(file) => Visitor {
            findings: &mut findings,
        }
        .visit_file(&file),
        Err(err) => {
            findings.insert(Finding {
                line: err.span().start().line,
                message: format!("does not parse: {err}"),
            });
        }
    }
    // Paths inside macro calls are plain tokens to syn, so read the tokens as well.
    if let Ok(tokens) = TokenStream::from_str(source) {
        let mut flat = Vec::new();
        flatten(tokens, &mut flat);
        scan_tokens(&flat, &mut findings);
    }
    findings.into_iter().collect()
}

fn io_path_message(path: &str) -> String {
    format!(
        "uses `{path}`; I/O-free crates must not reach files, the network, processes, \
         the environment, threads or the clock"
    )
}

fn is_io_path(path: &str) -> bool {
    IO_PATHS
        .iter()
        .any(|io| path == *io || path.starts_with(&format!("{io}::")))
}

/// Files reached through `include!` or `#[path]` must stay inside `src/`, where every file is
/// scanned: a relative path with no `..`.
fn stays_inside(path: &str) -> bool {
    let path = Path::new(path);
    path.is_relative()
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_) | Component::CurDir))
}

fn outside_message(what: &str, path: &str) -> String {
    format!(
        "{what} \"{path}\" may reach a file outside src/; I/O-free crates include only \
         files inside src/, by a relative path without `..`"
    )
}

/// A glob import `base::*` is I/O when `base` is I/O or contains an I/O module.
fn is_io_glob(base: &str) -> bool {
    is_io_path(base)
        || IO_PATHS
            .iter()
            .any(|io| io.starts_with(&format!("{base}::")))
}

struct Visitor<'f> {
    findings: &'f mut BTreeSet<Finding>,
}

impl Visitor<'_> {
    fn add(&mut self, span: Span, message: String) {
        self.findings.insert(Finding {
            line: span.start().line,
            message,
        });
    }
}

impl<'ast> Visit<'ast> for Visitor<'_> {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        let mut paths = Vec::new();
        flatten_use(&item.tree, "", &mut paths);
        for used in paths {
            let path = used.path.trim_end_matches("::self");
            if let Some(base) = path.strip_suffix("::*") {
                if is_io_glob(base) {
                    self.add(used.span, io_path_message(path));
                }
            } else if used.renamed && path == "std" {
                self.add(
                    used.span,
                    "renames `std`, which hides what the crate uses".to_owned(),
                );
            } else if is_io_path(path) {
                self.add(used.span, io_path_message(path));
            }
        }
        visit::visit_item_use(self, item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        // A `#![no_std]` crate needs `extern crate alloc` for heap types; `core` is harmless.
        if item.ident != "alloc" && item.ident != "core" {
            self.add(
                item.span(),
                format!(
                    "declares `extern crate {}`; I/O-free crates use only core and alloc",
                    item.ident
                ),
            );
        }
        visit::visit_item_extern_crate(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        for attr in item
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("path"))
        {
            let literal = match &attr.meta {
                syn::Meta::NameValue(value) => match &value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(path),
                        ..
                    }) => Some(path.value()),
                    _ => None,
                },
                _ => None,
            };
            match literal {
                Some(path) if stays_inside(&path) => {}
                Some(path) => self.add(attr.span(), outside_message("#[path]", &path)),
                None => self.add(
                    attr.span(),
                    "#[path] without a plain string cannot be checked".to_owned(),
                ),
            }
        }
        visit::visit_item_mod(self, item);
    }

    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
        self.add(
            item.span(),
            "declares an `extern` block; foreign functions can do any I/O".to_owned(),
        );
        visit::visit_item_foreign_mod(self, item);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if let Some(last) = mac.path.segments.last() {
            let name = last.ident.to_string();
            if IO_MACROS.contains(&name.as_str()) {
                self.add(
                    mac.path.span(),
                    format!("calls `{name}!`, which writes to the console"),
                );
            }
            if name == "include" {
                match mac.parse_body::<syn::LitStr>() {
                    Ok(path) if stays_inside(&path.value()) => {}
                    Ok(path) => {
                        self.add(mac.path.span(), outside_message("include!", &path.value()))
                    }
                    Err(_) => self.add(
                        mac.path.span(),
                        "include! with a computed path cannot be checked; I/O-free crates \
                         include only files inside src/"
                            .to_owned(),
                    ),
                }
            }
        }
        visit::visit_macro(self, mac);
    }
}

struct UsedPath {
    path: String,
    span: Span,
    renamed: bool,
}

fn flatten_use(tree: &syn::UseTree, prefix: &str, out: &mut Vec<UsedPath>) {
    let join = |name: &str| {
        if prefix.is_empty() {
            name.to_owned()
        } else {
            format!("{prefix}::{name}")
        }
    };
    match tree {
        syn::UseTree::Path(path) => flatten_use(&path.tree, &join(&path.ident.to_string()), out),
        syn::UseTree::Name(name) => out.push(UsedPath {
            path: join(&name.ident.to_string()),
            span: name.ident.span(),
            renamed: false,
        }),
        syn::UseTree::Rename(rename) => out.push(UsedPath {
            path: join(&rename.ident.to_string()),
            span: rename.ident.span(),
            renamed: true,
        }),
        syn::UseTree::Glob(glob) => out.push(UsedPath {
            path: join("*"),
            span: glob.span(),
            renamed: false,
        }),
        syn::UseTree::Group(group) => {
            for item in &group.items {
                flatten_use(item, prefix, out);
            }
        }
    }
}

enum Token {
    Ident(String, Span),
    PathSep,
    Other,
}

fn flatten(stream: TokenStream, out: &mut Vec<Token>) {
    let mut tokens = stream.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(group) => {
                out.push(Token::Other);
                flatten(group.stream(), out);
                out.push(Token::Other);
            }
            TokenTree::Ident(ident) => out.push(Token::Ident(ident.to_string(), ident.span())),
            TokenTree::Punct(punct)
                if punct.as_char() == ':' && punct.spacing() == Spacing::Joint =>
            {
                if matches!(tokens.peek(), Some(TokenTree::Punct(next)) if next.as_char() == ':') {
                    tokens.next();
                    out.push(Token::PathSep);
                } else {
                    out.push(Token::Other);
                }
            }
            _ => out.push(Token::Other),
        }
    }
}

fn scan_tokens(tokens: &[Token], findings: &mut BTreeSet<Finding>) {
    for (start, token) in tokens.iter().enumerate() {
        let Token::Ident(first, span) = token else {
            continue;
        };
        if first != "std" {
            continue;
        }
        let mut path = first.clone();
        let mut next = start + 1;
        while let (Some(Token::PathSep), Some(Token::Ident(segment, _))) =
            (tokens.get(next), tokens.get(next + 1))
        {
            path.push_str("::");
            path.push_str(segment);
            next += 2;
        }
        // Report the shortest I/O prefix, the same way the `use` check does.
        if let Some(io) = IO_PATHS
            .iter()
            .find(|io| path == **io || path.starts_with(&format!("{io}::")))
        {
            findings.insert(Finding {
                line: span.start().line,
                message: io_path_message(io),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(source: &str) -> Vec<(usize, String)> {
        scan_source(source)
            .into_iter()
            .map(|finding| (finding.line, finding.message))
            .collect()
    }

    fn lines(source: &str) -> Vec<usize> {
        let mut lines: Vec<usize> = found(source).into_iter().map(|(line, _)| line).collect();
        lines.dedup();
        lines
    }

    #[test]
    fn flags_io_modules_in_imports_and_expressions() {
        let source = "use std::fs;\n\
                      fn f() { let _ = std::env::var(\"X\"); }\n\
                      fn g() -> std::path::PathBuf { todo!() }\n";
        assert_eq!(lines(source), vec![1, 2, 3]);
    }

    #[test]
    fn flags_grouped_renamed_and_glob_imports() {
        let source = "use std::{collections::HashMap, io::Read};\n\
                      use std::net as network;\n\
                      use std::*;\n\
                      use std::time::{self, Instant};\n\
                      use ::std as hidden;\n";
        assert_eq!(lines(source), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn flags_io_inside_macro_calls() {
        let source = "fn f() { let _ = vec![std::process::id()]; }\n";
        assert_eq!(lines(source), vec![1]);
    }

    #[test]
    fn flags_console_macros_extern_blocks_and_extern_crates() {
        let source = "fn f() { println!(\"x\"); }\n\
                      fn g() { dbg!(1); }\n\
                      extern \"C\" { fn getpid() -> i32; }\n\
                      extern crate std as other;\n";
        assert_eq!(lines(source), vec![1, 2, 3, 4]);
    }

    #[test]
    fn flags_includes_and_module_paths_that_may_leave_src() {
        let source = "include!(\"../outside.rs\");\n\
                      include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));\n\
                      #[path = \"../../elsewhere/io.rs\"] mod io;\n\
                      #[path = \"/tmp/io.rs\"] mod absolute;\n\
                      include!(\"/tmp/io.rs\");\n";
        assert_eq!(lines(source), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn allows_pure_code() {
        let source = "use std::collections::BTreeMap;\n\
                      use core::fmt::Write as _;\n\
                      use std::time::Duration;\n\
                      pub fn later(d: Duration) -> std::time::Duration { d * 2 }\n\
                      pub fn text() -> String { format!(\"{}\", 1) }\n\
                      mod path { pub fn io() {} }\n\
                      fn local() { path::io(); }\n\
                      extern crate alloc;\n\
                      include!(\"tables/generated.rs\");\n\
                      #[path = \"tasks/model.rs\"] mod model;\n";
        assert_eq!(found(source), vec![]);
    }

    #[test]
    fn reads_crate_attributes() {
        assert_eq!(
            crate_attributes("#![no_std]\n#![forbid(unsafe_code)]\n"),
            CrateAttributes {
                forbids_unsafe_code: true,
                no_std: true
            }
        );
        assert_eq!(
            crate_attributes("#![forbid(missing_docs, unsafe_code)]\n"),
            CrateAttributes {
                forbids_unsafe_code: true,
                no_std: false
            }
        );
        // Conditional or weaker forms do not count.
        assert_eq!(
            crate_attributes(
                "#![cfg_attr(not(test), no_std)]\n#![deny(unsafe_code)]\n#![forbid(unsafe_op_in_unsafe_fn)]\n"
            ),
            CrateAttributes::default()
        );
    }
}
