//! Boundary checks for the crates of this workspace.
//!
//! Each crate states its own rules in its `tests/boundary.rs`, so `cargo test` fails as soon as
//! the crate depends on, or (for I/O-free crates) uses, something its rules do not allow.
//! Only normal dependencies count: dev- and build-dependencies never reach the shipped code.

mod scan;

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, PackageId, TargetKind};

pub use scan::{Finding, scan_source};

/// Async runtimes and the crates they are built on.
pub const ASYNC_RUNTIMES: &[&str] = &[
    "async-executor",
    "async-io",
    "async-std",
    "mio",
    "smol",
    "tokio",
];

/// Database drivers, pools and ORMs.
pub const DATABASE_CRATES: &[&str] = &[
    "bb8-postgres",
    "deadpool-postgres",
    "diesel",
    "postgres",
    "sea-orm",
    "sqlx",
    "sqlx-core",
    "sqlx-postgres",
    "tokio-postgres",
];

/// HTTP servers, clients and web frameworks.
pub const WEB_CRATES: &[&str] = &[
    "actix-web",
    "axum",
    "axum-core",
    "hyper",
    "hyper-util",
    "poem",
    "reqwest",
    "rocket",
    "salvo",
    "tower-http",
    "ureq",
    "warp",
];

/// Rules for one workspace crate. Build them with [`Boundary::of`] and run [`Boundary::check`].
#[derive(Debug, Clone)]
pub struct Boundary {
    package: String,
    workspace: Vec<String>,
    external: Option<Vec<String>>,
    forbidden: Vec<String>,
    forbidden_direct: Vec<String>,
    no_dependents: bool,
    io_free: bool,
}

impl Boundary {
    /// Starts the rules for the workspace package `package`. By default it may depend on no
    /// other workspace crate and on any external crate.
    pub fn of(package: &str) -> Self {
        Self {
            package: package.to_owned(),
            workspace: Vec::new(),
            external: None,
            forbidden: Vec::new(),
            forbidden_direct: Vec::new(),
            no_dependents: false,
            io_free: false,
        }
    }

    /// The workspace crates this crate may depend on directly.
    pub fn may_use_workspace(mut self, crates: &[&str]) -> Self {
        self.workspace = owned(crates);
        self
    }

    /// The only external crates this crate may depend on directly.
    pub fn may_use_only_external(mut self, crates: &[&str]) -> Self {
        self.external = Some(owned(crates));
        self
    }

    /// Crates that must not appear anywhere in this crate's dependency tree.
    pub fn must_not_use(mut self, crates: &[&str]) -> Self {
        self.forbidden.extend(owned(crates));
        self
    }

    /// Crates this crate must not depend on directly (they may still arrive through a
    /// workspace crate that owns them).
    pub fn must_not_use_directly(mut self, crates: &[&str]) -> Self {
        self.forbidden_direct.extend(owned(crates));
        self
    }

    /// No other workspace crate may depend on this one.
    pub fn without_dependents(mut self) -> Self {
        self.no_dependents = true;
        self
    }

    /// No I/O: no async runtime, database or web crate in the dependency tree, and no file,
    /// network, process, environment, thread, clock or console access in the sources.
    pub fn io_free(mut self) -> Self {
        self.io_free = true;
        self.forbidden.extend(owned(ASYNC_RUNTIMES));
        self.forbidden.extend(owned(DATABASE_CRATES));
        self.forbidden.extend(owned(WEB_CRATES));
        self
    }

    /// Panics with every broken rule.
    pub fn check(&self) {
        let violations = self.violations(workspace_metadata());
        assert!(
            violations.is_empty(),
            "{} breaks its boundary:\n  - {}\n(the rules are in the crate's tests/boundary.rs)",
            self.package,
            violations.join("\n  - ")
        );
    }

    /// Every broken rule, checked against `metadata`.
    pub fn violations(&self, metadata: &Metadata) -> Vec<String> {
        let graph = Graph::new(metadata);
        let Some(id) = graph.workspace_package(&self.package) else {
            return vec![format!("no workspace package is named `{}`", self.package)];
        };
        let mut violations = Vec::new();

        for dep in graph.direct(id) {
            let name = graph.name(dep);
            if graph.is_workspace(dep) {
                if !self.workspace.iter().any(|allowed| allowed == name) {
                    violations.push(format!(
                        "depends on workspace crate `{name}`; allowed: {}",
                        list(&self.workspace)
                    ));
                }
            } else if let Some(external) = &self.external
                && !external.iter().any(|allowed| allowed == name)
            {
                violations.push(format!(
                    "depends on external crate `{name}`; allowed: {}",
                    list(external)
                ));
            }
            if self.forbidden_direct.iter().any(|f| f == name) {
                violations.push(format!(
                    "depends directly on `{name}`, which this crate must reach only through \
                     the workspace crate that owns it"
                ));
            }
        }

        for (dep, chain) in graph.closure(id) {
            let name = graph.name(dep);
            if self.forbidden.iter().any(|f| f == name) {
                violations.push(format!(
                    "pulls in `{name}` ({}), which this crate must not use",
                    chain.join(" -> ")
                ));
            }
        }

        if self.no_dependents {
            for other in graph.workspace_members() {
                if graph.direct(other).any(|dep| dep == id) {
                    violations.push(format!(
                        "`{}` depends on it; nothing may depend on `{}`",
                        graph.name(other),
                        self.package
                    ));
                }
            }
        }

        if self.io_free {
            for file in graph.source_files(id) {
                let source = match std::fs::read_to_string(&file) {
                    Ok(source) => source,
                    Err(err) => {
                        violations.push(format!("cannot read {}: {err}", file.display()));
                        continue;
                    }
                };
                let shown = graph.relative(&file);
                for finding in scan_source(&source) {
                    violations.push(format!(
                        "{}:{} {}",
                        shown.display(),
                        finding.line,
                        finding.message
                    ));
                }
            }
        }

        violations
    }
}

/// The metadata of the workspace this crate belongs to, loaded once per test process.
pub fn workspace_metadata() -> &'static Metadata {
    static METADATA: OnceLock<Metadata> = OnceLock::new();
    METADATA.get_or_init(|| {
        MetadataCommand::new()
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .other_options(vec![
                "--locked".to_owned(),
                "--filter-platform".to_owned(),
                env!("BOUNDARIES_TARGET").to_owned(),
            ])
            .exec()
            .expect("`cargo metadata` failed")
    })
}

struct Graph<'a> {
    metadata: &'a Metadata,
    deps: HashMap<&'a PackageId, Vec<&'a PackageId>>,
}

impl<'a> Graph<'a> {
    fn new(metadata: &'a Metadata) -> Self {
        let resolve = metadata
            .resolve
            .as_ref()
            .expect("cargo metadata was run with dependencies");
        let deps = resolve
            .nodes
            .iter()
            .map(|node| {
                let normal = node
                    .deps
                    .iter()
                    .filter(|dep| {
                        dep.dep_kinds
                            .iter()
                            .any(|kind| kind.kind == DependencyKind::Normal)
                    })
                    .map(|dep| &dep.pkg)
                    .collect();
                (&node.id, normal)
            })
            .collect();
        Self { metadata, deps }
    }

    fn workspace_package(&self, name: &str) -> Option<&'a PackageId> {
        self.metadata
            .workspace_packages()
            .into_iter()
            .find(|package| package.name.as_str() == name)
            .map(|package| &package.id)
    }

    fn workspace_members(&self) -> impl Iterator<Item = &'a PackageId> + '_ {
        self.metadata.workspace_members.iter()
    }

    fn is_workspace(&self, id: &PackageId) -> bool {
        self.metadata.workspace_members.contains(id)
    }

    fn name(&self, id: &PackageId) -> &'a str {
        self.metadata[id].name.as_str()
    }

    fn direct(&self, id: &PackageId) -> impl Iterator<Item = &'a PackageId> + '_ {
        self.deps.get(id).into_iter().flatten().copied()
    }

    /// Every package in the normal dependency tree, with the chain of names that leads to it.
    fn closure(&self, root: &'a PackageId) -> Vec<(&'a PackageId, Vec<&'a str>)> {
        let mut seen = BTreeSet::from([root]);
        let mut queue = VecDeque::from([(root, vec![self.name(root)])]);
        let mut found = Vec::new();
        while let Some((id, chain)) = queue.pop_front() {
            for dep in self.direct(id) {
                if seen.insert(dep) {
                    let mut next = chain.clone();
                    next.push(self.name(dep));
                    found.push((dep, next.clone()));
                    queue.push_back((dep, next));
                }
            }
        }
        found
    }

    /// The Rust files under the source directories of the package's library and binaries.
    fn source_files(&self, id: &PackageId) -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = self.metadata[id]
            .targets
            .iter()
            .filter(|target| {
                target
                    .kind
                    .iter()
                    .any(|kind| matches!(kind, TargetKind::Lib | TargetKind::Bin))
            })
            .filter_map(|target| target.src_path.parent())
            .map(|dir| dir.as_std_path().to_path_buf())
            .collect();
        dirs.sort();
        dirs.dedup();
        let mut files = Vec::new();
        for dir in dirs {
            collect_rust_files(&dir, &mut files);
        }
        files.sort();
        files.dedup();
        files
    }

    fn relative<'p>(&self, path: &'p Path) -> &'p Path {
        path.strip_prefix(self.metadata.workspace_root.as_std_path())
            .unwrap_or(path)
    }
}

fn collect_rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

fn owned(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn list(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_owned()
    } else {
        items
            .iter()
            .map(|item| format!("`{item}`"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
