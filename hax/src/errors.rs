#![allow(unused_doc_comment)]

use std::io;

error_chain! {
	foreign_links {
		Io(io::Error) #[doc = "I/O error when communicating with a HAX device."];
	}
}

pub (in super) fn last_os_error() -> Error {
	Error::from(io::Error::last_os_error())
}
