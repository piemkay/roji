//! Turns the text a person types into domain values, with no I/O.
//!
//! Like the domain core, this crate never touches files, the network, a database, the
//! environment or the clock, so parsing gives the same result wherever it runs.
//! `tests/boundary.rs` enforces this.
