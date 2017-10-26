use {nix, Result, RawFd};
use std::io;

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

pub macro kvm_ioctl_none_arg($name:ident with $code:expr) {
    pub unsafe fn $name(fd: RawFd, data: i32) -> Result {
        let code = io!(KVM_IO, $code);

        convert(nix::Errno::result(nix::libc::ioctl(fd, code, data)))
    }
}

pub macro kvm_ioctl_none($name:ident with $code:expr) {
    pub unsafe fn $name(fd: RawFd) -> Result {
        ioctl!(none $name with KVM_IO, $code);
        convert($name(fd))
    }
}

pub macro kvm_ioctl_rw($name:ident with $code:expr; $st:ty) {
    pub unsafe fn $name(fd: RawFd, data: *mut $st) -> Result {
        ioctl!(readwrite $name with KVM_IO, $code; $st);
        convert($name(fd, data))
    }
}
