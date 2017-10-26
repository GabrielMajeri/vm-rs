//! Errors returned when opening a KVM device.
#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        UnsupportedVersion(expected: u32, current: u32) {
            description("unsupported API version")
            display("unsupported KVM API version: expected {}, found {}", expected, current)
        }

        UnsupportedCapability(name: String) {
            description("unsupported capability")
            display("KVM capability not supported: {}", name)
        }
    }
}
