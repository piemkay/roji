//! The domain core of Roji: its types and rules, with no I/O.
//!
//! Nothing here reads files, the network, a database, the environment or the clock, and no
//! async runtime is involved, so the same rules can later run on the server, in the browser
//! and in native apps. `tests/boundary.rs` enforces this.
