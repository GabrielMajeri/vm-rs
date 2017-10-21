use {RawFd, KVM_IO, ioctl_none};
use ext;

pub trait Object {
    fn fd(&self) -> RawFd;

    /// Checks if a given extension is supported by KVM.
    fn extension_supported(&self, extension: ext::Extension) -> u32 {
        let code = io!(KVM_IO, 0x03);

        let supported = ioctl_none(self.fd(), code, extension as u32);

        match supported {
            Ok(value) => value as u32,
            _ => 0,
        }
    }
}
