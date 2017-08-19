# vm-rs

Support for building virtual machines using Rust.

This project aims to provide a virtual machine emulator similar to [QEMU](https://www.qemu.org/).

## Supported CPU architectures
- x86 / AMD64

## Supported supervisors
- Intel's Hardware Accelerated Execution (partial): only on Intel processors who have [SLAT](https://en.wikipedia.org/wiki/Second_Level_Address_Translation).
Requires the [HAX manager](https://software.intel.com/en-us/android/articles/intel-hardware-accelerated-execution-manager) to be installed.
Currently, only Windows is supported.
