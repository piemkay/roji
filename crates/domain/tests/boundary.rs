//! The domain core is I/O-free and depends on no other crate of the workspace.

use boundaries::Boundary;

#[test]
fn domain_stays_io_free() {
    Boundary::of("roji-domain")
        .may_use_workspace(&[])
        .may_use_only_external(&[])
        .io_free()
        .check();
}
