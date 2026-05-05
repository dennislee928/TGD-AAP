//! quantum_bridge/mod.rs — Rust interface to holyQASM quantum logic.
//!
//! Provides a safe Rust API for scheduling and retrieving results from
//! holyQASM quantum circuit simulations via FFI or subprocess calls.

pub mod circuit;
pub mod runner;
