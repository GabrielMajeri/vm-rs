#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

/// Abstraction of a computer bus, which allows various component to interconnect and communicate.
pub trait Bus {
    /// Trait implemented by all devices on this bus.
    type Child;
}
