//! Guard tests for the I/O-free rules. Each builds a throwaway workspace that breaks the rules
//! in a way an earlier version of the check missed, and asserts that the check now reports it.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use boundaries::{Boundary, metadata_for};

const CLEAN: &str = "#![no_std]\n#![forbid(unsafe_code)]\nextern crate alloc;\n\
                     pub fn double(x: u32) -> u32 { x * 2 }\n";

/// Removes the throwaway workspace when the test ends.
struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("a parent directory")).expect("create directory");
    fs::write(path, contents).expect("write file");
}

/// The violations of an I/O-free crate `pure` whose manifest ends with `manifest_extra`, whose
/// `src/lib.rs` is `lib_rs` and which has `files` (paths relative to the crate). Next to the
/// workspace, outside it, is a path crate named `tokio` that `pure` can depend on.
fn violations(manifest_extra: &str, lib_rs: &str, files: &[(&str, &str)]) -> Vec<String> {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "boundaries-guard-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _fixture = Fixture(root.clone());

    write(
        &root.join("ws/Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"pure\"]\n",
    );
    write(
        &root.join("ws/pure/Cargo.toml"),
        &format!(
            "[package]\nname = \"pure\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n{manifest_extra}"
        ),
    );
    write(&root.join("ws/pure/src/lib.rs"), lib_rs);
    for (path, contents) in files {
        write(&root.join("ws/pure").join(path), contents);
    }
    write(
        &root.join("vendor/tokio/Cargo.toml"),
        "[package]\nname = \"tokio\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
    );
    write(&root.join("vendor/tokio/src/lib.rs"), "");

    // The same loader as the real checks, so a platform filter added there fails here.
    let metadata = metadata_for(&root.join("ws/Cargo.toml"), &["--offline"]);
    Boundary::of("pure")
        .may_use_only_external(&[])
        .io_free()
        .violations(&metadata)
}

fn assert_reports(violations: &[String], expected: &str) {
    assert!(
        violations
            .iter()
            .any(|violation| violation.contains(expected)),
        "expected a violation containing {expected:?}, got {violations:#?}"
    );
}

#[test]
fn a_clean_no_std_crate_passes() {
    assert_eq!(violations("", CLEAN, &[]), Vec::<String>::new());
}

#[test]
fn a_dependency_for_webassembly_only_is_seen() {
    let found = violations(
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]\n\
         tokio = { path = \"../../vendor/tokio\" }\n",
        CLEAN,
        &[],
    );
    assert_reports(&found, "pulls in `tokio`");
}

#[test]
fn std_reached_through_a_module_alias_is_stopped_by_no_std() {
    for lib_rs in [
        "#![forbid(unsafe_code)]\nuse std::time;\npub fn now() { let _ = time::Instant::now(); }\n",
        "#![forbid(unsafe_code)]\nuse std::time as t;\npub fn now() { let _ = t::Instant::now(); }\n",
    ] {
        assert_reports(&violations("", lib_rs, &[]), "must declare #![no_std]");
    }
}

#[test]
fn std_brought_back_into_a_no_std_crate_is_flagged() {
    let lib_rs = "#![no_std]\n#![forbid(unsafe_code)]\nextern crate std;\nuse std::time as t;\n\
                  pub fn now() { let _ = t::Instant::now(); }\n";
    assert_reports(&violations("", lib_rs, &[]), "extern crate std");
}

#[test]
fn code_included_from_outside_src_is_flagged() {
    let outside = [(
        "outside.rs",
        "extern crate std;\npub fn io() { let _ = std::fs::read(\"x\"); }\n",
    )];
    let included = format!("{CLEAN}include!(\"../outside.rs\");\n");
    assert_reports(
        &violations("", &included, &outside),
        "include! \"../outside.rs\"",
    );
    let module = format!("{CLEAN}#[path = \"../outside.rs\"]\nmod outside;\n");
    assert_reports(
        &violations("", &module, &outside),
        "#[path] \"../outside.rs\"",
    );
}

#[test]
fn a_crate_root_without_forbid_unsafe_code_is_flagged() {
    let lib_rs = "#![no_std]\npub fn double(x: u32) -> u32 { x * 2 }\n";
    assert_reports(
        &violations("", lib_rs, &[]),
        "must declare #![forbid(unsafe_code)]",
    );
}
