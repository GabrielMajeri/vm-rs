//! Contains structures and functions for the x86 architecture.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate bitflags;

pub mod state;

pub mod fpu;

pub mod msr;

pub mod vmx;
