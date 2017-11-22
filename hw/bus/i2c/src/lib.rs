//! Inter-Integrated Circuit (I²C) hardware.
//!
//! Conforms to the [I²C-bus Specification](http://i2c.info/i2c-bus-specification).
//! Further documentation available [here](https://www.i2c-bus.org/).

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

//! Unique identifier of a device.
//!
//! An address is used to select which device(s) you are communicating with.
//!
//! An address is normally 7 bit long, although an extension exists for 10 bit addresses.
//! Note that some addresses are reserved, so there might be
//! less than 2<sup>7</sup> or 2<sup>10</sup> available addresses.
//!
//! There are some special addresses, such as for broadcasting,
//! and some reserved by the specification.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Address(u16);

pub const BROADCAST: Address = Address(0);
