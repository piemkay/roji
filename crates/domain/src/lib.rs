//! The domain core of Roji: its types and rules, with no I/O.
//!
//! The crate is `no_std`: it has only `core` and `alloc`, so nothing here can read files, the
//! network, a database, the environment or the clock, and no async runtime is involved. The
//! same rules can later run on the server, in the browser and in native apps.
//! `tests/boundary.rs` enforces this.

#![no_std]
#![forbid(unsafe_code)]

// Heap types such as String, Vec and BTreeMap come from alloc.
extern crate alloc;
