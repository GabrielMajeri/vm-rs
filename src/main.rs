//! Virtual machine framework in Rust.

#![cfg_attr(feature = "cargo-clippy", deny(warnings, missing_docs, clippy))]

extern crate env_logger;

extern crate kvm;

fn main() {
	env_logger::init().expect("Failed to initialize logger");
}
