//! System call used to interact with the KVM kernel module.

use structs;

use {nix, RawFd};
use std::io;
use errors::Result;

/// The type of all KVM `ioctl`s.
const KVM_IO: u8 = 0xAE;

/// Converts a `nix::Result` to `Result<i32, io::Error>`.
pub fn convert(result: nix::Result<nix::libc::c_int>) -> Result {
    result.map(|value| value as u32).map_err(|err| {
        let errno = match err {
            nix::Error::Sys(errno) => errno,
            _ => unreachable!(),
        };

        io::Error::from_raw_os_error(errno as i32)
    })
}

macro_rules! kvm_ioctl {
    (none_arg $name:ident with $code:expr) => {
        pub unsafe fn $name(fd: RawFd, data: i32) -> Result {
            let code = io!(KVM_IO, $code);

            convert(nix::Errno::result(nix::libc::ioctl(fd, code, data)))
        }
    };
    (none $name:ident with $code:expr) => {
        pub unsafe fn $name(fd: RawFd) -> Result {
            ioctl!(none $name with KVM_IO, $code);
            convert($name(fd))
        }
    };
    (read $name:ident with $code:expr; $st:ty) => {
        pub unsafe fn $name(fd: RawFd, data: *mut $st) -> Result {
            ioctl!(read $name with KVM_IO, $code; $st);
            convert($name(fd, data))
        }
    };
    (write_ptr $name:ident with $code:expr; $st:ty) => {
        pub unsafe fn $name(fd: RawFd, data: *mut $st) -> Result {
            ioctl!(write_ptr $name with KVM_IO, $code; $st);
            convert($name(fd, data))
        }
    };
    (readwrite $name:ident with $code:expr; $st:ty) => {
        pub unsafe fn $name(fd: RawFd, data: *mut $st) -> Result {
            ioctl!(readwrite $name with KVM_IO, $code; $st);
            convert($name(fd, data))
        }
    };
}

kvm_ioctl!(none_arg get_api_version with 0x00);
kvm_ioctl!(none_arg check_extension with 0x03);
kvm_ioctl!(none_arg create_vm with 0x01);

kvm_ioctl!(none get_vcpu_mmap_size with 0x04);

kvm_ioctl!(readwrite get_emulated_cpuid with 0x09; structs::cpuid::CpuidHeader);

kvm_ioctl!(none create_irq_chip with 0x60);
kvm_ioctl!(readwrite get_irq_chip with 0x62; structs::irq::IrqChip);

kvm_ioctl!(none_arg create_vcpu with 0x41);

kvm_ioctl!(read get_regs with 0x81; structs::state::Registers);
kvm_ioctl!(write_ptr set_regs with 0x82; structs::state::Registers);

kvm_ioctl!(read get_fpu with 0x8C; structs::fpu::FpuState);
kvm_ioctl!(write_ptr set_fpu with 0x8D; structs::fpu::FpuState);
