//! Parsing is I/O-free and may use only the domain core from the workspace.

use boundaries::Boundary;

#[test]
fn parse_stays_io_free() {
    Boundary::of("roji-parse")
        .may_use_workspace(&["roji-domain"])
        .may_use_only_external(&[])
        .io_free()
        .check();
}
