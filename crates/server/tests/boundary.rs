//! The server is the outer layer: it may use every other crate of the workspace, reaches the
//! database only through roji-store, and nothing depends on it.

use boundaries::{Boundary, DATABASE_CRATES};

#[test]
fn server_is_the_outer_layer() {
    Boundary::of("roji-server")
        .may_use_workspace(&["roji-domain", "roji-parse", "roji-store"])
        .must_not_use_directly(DATABASE_CRATES)
        .without_dependents()
        .check();
}
