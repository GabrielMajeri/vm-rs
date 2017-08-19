# x86 arch support

This crate contains structure for the x86 architecture, i.e. all processors starting with the Intel 8086,
up the the newest models.

To simplify the implementation, these structures have support for AMD64, meaning they natively use 64-bit pointers
and registers. A 32-bit virtual machine needs only to ignore 64-bit support.
