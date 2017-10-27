use accel;
use accel::errors::Result;
use vm::VirtualMachine;
use std::fs::File;
use x86::state::State;
use kvm;
use kvm::RawFd;

pub struct VirtualCPU<'a> {
    vm: &'a VirtualMachine<'a>,
    file: File,
}

impl<'a> VirtualCPU<'a> {
    /// Initializes the virtual CPU.
    pub fn new(vm: &'a VirtualMachine, file: File) -> Result<Self> {
        let vcpu = VirtualCPU { vm, file };

        Ok(vcpu)
    }

    /// Retrieves the raw file descriptor for this device.
    #[inline]
    fn fd(&self) -> RawFd {
        use std::os::unix::io::AsRawFd;
        self.file.as_raw_fd()
    }

    fn set_regs(&self, state: &State) -> Result<()> {
        let mut regs = kvm::structs::state::Registers::default();
        let r = &state.r;

        regs.r[0] = r[0];
        regs.r[2] = r[1];
        regs.r[3] = r[2];
        regs.r[1] = r[3];

        regs.r[6] = r[4];
        regs.r[7] = r[5];

        regs.r[4] = r[6];
        regs.r[5] = r[7];

        regs.r[8..].copy_from_slice(&r[8..]);

        regs.ip = state.ip;
        regs.flags = state.flags.bits();

        unsafe { kvm::ioctl::set_regs(self.fd(), &mut regs)? };

        Ok(())
    }

    fn get_regs(&self, state: &mut State) -> Result<()> {
        let regs = {
            let mut regs = kvm::structs::state::Registers::default();

            unsafe { kvm::ioctl::get_regs(self.fd(), &mut regs)? };

            regs
        };

        let r = &mut state.r;

        r[0] = regs.r[0];
        r[1] = regs.r[2];
        r[2] = regs.r[3];
        r[3] = regs.r[1];

        r[4] = regs.r[6];
        r[5] = regs.r[7];

        r[6] = regs.r[4];
        r[7] = regs.r[5];

        r[8..].copy_from_slice(&regs.r[8..]);

        state.ip = regs.ip;
        use x86::state::Flags;
        state.flags = Flags::from_bits(regs.flags).unwrap();

        Ok(())
    }
}

impl<'a> accel::VirtualCPU<'a> for VirtualCPU<'a> {
    fn sync(&self, state: &mut State, set: bool) -> Result<()> {
        if set {
            self.set_regs(state)
        } else {
            self.get_regs(state)
        }
    }
}
