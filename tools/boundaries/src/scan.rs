//! Finds I/O, and ways to hide it, in the Rust source of an I/O-free crate: paths into the `std`
//! modules that reach files, the network, processes, the environment, threads or the clock;
//! console macros; `extern` blocks; extern crates other than `alloc` and `core`; macros defined
//! with `macro_rules!`; `include!`, `include_str!` and `include_bytes!` under any name; and
//! `path =` in any attribute. The rules run on the whole token stream of each file, so they also
//! see macro bodies and attribute arguments such as `cfg_attr`.
//!
//! Also reads the attributes of a crate root.

use std::collections::BTreeSet;
use std::str::FromStr;

use proc_macro2::{Delimiter, Spacing, Span, TokenStream, TokenTree};
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

/// Macros that read other files into the crate at compile time.
pub const INCLUDE_MACROS: &[&str] = &["include", "include_bytes", "include_str"];

/// The only crates an I/O-free crate may declare with `extern crate`.
const ALLOWED_EXTERN_CRATES: &[&str] = &["alloc", "core"];

/// One piece of I/O, or of hidden code, found in a source file.
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

/// Every finding in `source`, one per line and kind.
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
    // syn sees macro bodies and attribute arguments only as tokens, so the rules that must
    // also hold there run on the token stream of the whole file.
    match TokenStream::from_str(source) {
        Ok(tokens) => {
            scan_token_rules(tokens.clone(), false, &mut findings);
            let mut flat = Vec::new();
            flatten(tokens, &mut flat);
            scan_std_paths(&flat, &mut findings);
        }
        Err(err) => {
            findings.insert(Finding {
                line: err.span().start().line,
                message: format!("does not tokenize: {err}"),
            });
        }
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

/// A glob import `base::*` is I/O when `base` is I/O or contains an I/O module.
fn is_io_glob(base: &str) -> bool {
    is_io_path(base)
        || IO_PATHS
            .iter()
            .any(|io| io.starts_with(&format!("{base}::")))
}

/// The name of an identifier without the `r#` of a raw identifier.
fn name_of(ident: &proc_macro2::Ident) -> String {
    let name = ident.to_string();
    name.strip_prefix("r#").map(str::to_owned).unwrap_or(name)
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
            if let Some(last) = path.rsplit("::").next()
                && INCLUDE_MACROS.contains(&last)
            {
                self.add(used.span, include_message(last));
            }
        }
        visit::visit_item_use(self, item);
    }

    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
        self.add(item.span(), extern_block_message());
        visit::visit_item_foreign_mod(self, item);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if let Some(last) = mac.path.segments.last() {
            let name = name_of(&last.ident);
            if IO_MACROS.contains(&name.as_str()) {
                self.add(
                    mac.path.span(),
                    format!("calls `{name}!`, which writes to the console"),
                );
            }
        }
        visit::visit_macro(self, mac);
    }
}

fn include_message(name: &str) -> String {
    format!("uses `{name}!`; I/O-free crates include no other files")
}

fn extern_block_message() -> String {
    "declares an `extern` block; foreign functions can do any I/O".to_owned()
}

fn extern_crate_message(name: &str) -> String {
    format!("declares `extern crate {name}`; I/O-free crates use only core and alloc")
}

fn macro_rules_message() -> String {
    "defines a macro with `macro_rules!`; I/O-free crates define no macros, so all of their \
     code stays visible to this check"
        .to_owned()
}

fn path_attribute_message() -> String {
    "sets a module file with `path =` in an attribute; I/O-free crates keep every module in \
     its default file under src/"
        .to_owned()
}

fn is_punct(token: Option<&TokenTree>, ch: char) -> bool {
    matches!(token, Some(TokenTree::Punct(punct)) if punct.as_char() == ch)
}

fn ident_name(token: Option<&TokenTree>) -> Option<String> {
    match token {
        Some(TokenTree::Ident(ident)) => Some(name_of(ident)),
        _ => None,
    }
}

/// Whether the tokens before a `[...]` group make it an attribute: `#[...]` or `#![...]`.
fn opens_attribute(before: &[TokenTree]) -> bool {
    match before {
        [.., TokenTree::Punct(hash), TokenTree::Punct(bang)]
            if hash.as_char() == '#' && bang.as_char() == '!' =>
        {
            true
        }
        [.., TokenTree::Punct(hash)] => hash.as_char() == '#',
        _ => false,
    }
}

/// The rules that must hold everywhere, macro bodies and attribute arguments included.
fn scan_token_rules(stream: TokenStream, in_attribute: bool, findings: &mut BTreeSet<Finding>) {
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut add = |span: Span, message: String| {
        findings.insert(Finding {
            line: span.start().line,
            message,
        });
    };
    let mut nested = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let next = tokens.get(index + 1);
        match token {
            TokenTree::Group(group) => {
                let attribute =
                    group.delimiter() == Delimiter::Bracket && opens_attribute(&tokens[..index]);
                nested.push((group.stream(), in_attribute || attribute));
            }
            TokenTree::Ident(ident) => {
                let name = name_of(ident);
                if INCLUDE_MACROS.contains(&name.as_str())
                    && (is_punct(next, '!') || ident_name(next).as_deref() == Some("as"))
                {
                    add(ident.span(), include_message(&name));
                } else if name == "macro_rules" && is_punct(next, '!') {
                    add(ident.span(), macro_rules_message());
                } else if name == "extern" {
                    if ident_name(next).as_deref() == Some("crate") {
                        if let Some(krate) = ident_name(tokens.get(index + 2))
                            && !ALLOWED_EXTERN_CRATES.contains(&krate.as_str())
                        {
                            add(ident.span(), extern_crate_message(&krate));
                        }
                    } else if matches!(next, Some(TokenTree::Literal(_)))
                        || matches!(next, Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace)
                    {
                        add(ident.span(), extern_block_message());
                    }
                } else if in_attribute
                    && name == "path"
                    && matches!(next, Some(TokenTree::Punct(eq)) if eq.as_char() == '=' && eq.spacing() == Spacing::Alone)
                {
                    add(ident.span(), path_attribute_message());
                }
            }
            _ => {}
        }
    }
    for (stream, in_attribute) in nested {
        scan_token_rules(stream, in_attribute, findings);
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
        syn::UseTree::Path(path) => flatten_use(&path.tree, &join(&name_of(&path.ident)), out),
        syn::UseTree::Name(name) => out.push(UsedPath {
            path: join(&name_of(&name.ident)),
            span: name.ident.span(),
            renamed: false,
        }),
        syn::UseTree::Rename(rename) => out.push(UsedPath {
            path: join(&name_of(&rename.ident)),
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
            TokenTree::Ident(ident) => out.push(Token::Ident(name_of(&ident), ident.span())),
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

fn scan_std_paths(tokens: &[Token], findings: &mut BTreeSet<Finding>) {
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
    fn flags_includes_module_paths_and_macro_definitions_in_any_form() {
        let source = "include!(\"tables.rs\");\n\
                      const T: &str = core::include_str!(\"../x.txt\");\n\
                      use core::include as inc;\n\
                      use core::{include_bytes};\n\
                      #[path = \"tasks/model.rs\"] mod model;\n\
                      #[cfg_attr(all(), path = \"../x.rs\")] mod x;\n\
                      macro_rules! hide { () => { extern crate std as s; } }\n\
                      fn f() { let _ = [r#include!(\"y.rs\")]; }\n\
                      mod inner { #![cfg_attr(all(), path = \"../z.rs\")] }\n";
        assert_eq!(lines(source), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
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
                      pub fn include(task: u32) -> u32 { let path = task; path }\n\
                      #[derive(Debug)] #[cfg(feature = \"path\")] pub struct Task;\n";
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
