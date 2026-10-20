//! The store owns the database: it may use the domain core, and serves or calls no HTTP.

use boundaries::{Boundary, WEB_CRATES};

#[test]
fn store_owns_only_the_database() {
    Boundary::of("roji-store")
        .may_use_workspace(&["roji-domain"])
        .must_not_use(WEB_CRATES)
        .check();
}
