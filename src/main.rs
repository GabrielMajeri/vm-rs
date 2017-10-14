//! Virtual machine framework in Rust.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate env_logger;

extern crate kvm;

fn main() {
    env_logger::init().expect("Failed to initialize logger");
}
