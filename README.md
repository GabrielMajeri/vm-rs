# vm-rs
Support for building virtual machines using Rust.

This project aims to provide virtual machine software similar to [QEMU][QEMU],
as well as bindings and libraries which support such a virtual machine emulator / manager.

[QEMU]: https://www.qemu.org/

## Supported CPU architectures
- x86 / AMD64

## Supported hardware-acceleration
- [Kernel-based Virtual Machine][kvm] (on Linux, Intel / AMD)
- [Hardware Accelerated Execution][hax] (on Windows, Intel only)

[kvm]: https://www.linux-kvm.org
[hax]: https://software.intel.com/en-us/articles/intel-hardware-accelerated-execution-manager-intel-haxm

## License
All source code in this repository is dual licensed under
[Apache-2.0][Apache-2.0] and [MIT][MIT].

You may use the software under either license, at your option.

[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT]: https://opensource.org/licenses/MIT
