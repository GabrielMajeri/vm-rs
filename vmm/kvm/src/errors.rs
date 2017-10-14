//! Error types returned by this crate.

use nix;
use std::io;

error_chain! {
    foreign_links {
        Io(io::Error) #[doc = "Error returned when interacting with KVM device."];
    }
}

impl From<nix::Error> for Error {
    fn from(error: nix::Error) -> Error {
        let errno = match error {
            nix::Error::Sys(errno) => errno,
            _ => nix::errno::EINVAL,
        };

        Error::from(io::Error::from_raw_os_error(errno as i32))
    }
}
