//! Turns the text a person types into domain values, with no I/O.
//!
//! Like the domain core, this crate is `no_std` and never touches files, the network, a
//! database, the environment or the clock, so parsing gives the same result wherever it runs.
//! `tests/boundary.rs` enforces this.

#![no_std]
#![forbid(unsafe_code)]

// Heap types such as String, Vec and BTreeMap come from alloc.
extern crate alloc;
