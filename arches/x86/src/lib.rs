//! Contains structures and functions for the x86 architecture.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

/// Structures representing the x86 processor state.
pub mod state;

/// Support for model-specific registers.
pub mod msr;

/// Structures and enums for Intel's Virtual Machine Extensions.
pub mod vmx;
