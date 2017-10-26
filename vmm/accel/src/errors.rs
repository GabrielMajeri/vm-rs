//! Error handling types.
#![allow(missing_docs)]

error_chain! {
    foreign_links {
        System(::std::io::Error);
    }
}
