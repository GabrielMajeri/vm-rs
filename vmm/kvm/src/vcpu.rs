use accel;
use accel::errors::Result;
use vm::VirtualMachine;
use std::fs::File;
use x86;
use x86::state::State;
use kvm;
use kvm::RawFd;
use kvm::structs::run;
use memmap as mm;
use std::mem;

pub struct VirtualCPU<'a> {
    vm: &'a VirtualMachine<'a>,
    file: File,
    run: mm::Mmap,
}

impl<'a> VirtualCPU<'a> {
    /// Initializes the virtual CPU.
    pub fn new(vm: &'a VirtualMachine, file: File) -> Result<Self> {
        let prot = mm::Protection::ReadWrite;
        let offset = 0;
        let len = 64;
        let run = mm::Mmap::open_with_offset(&file, prot, offset, len)?;

        let vcpu = VirtualCPU { vm, file, run };

        Ok(vcpu)
    }

    /// Retrieves the raw file descriptor for this device.
    #[inline]
    fn fd(&self) -> RawFd {
        use std::os::unix::io::AsRawFd;
        self.file.as_raw_fd()
    }

    /// Retrieves a reference to the `mmap`ed run state structure.
    #[inline]
    fn run_state(&self) -> &mut run::RunState {
        unsafe { mem::transmute(self.run.ptr()) }
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

    fn set_sregs(&self, state: &State) -> Result<()> {
        use self::kvm::structs::state;
        let mut sregs = state::SpecialRegisters::default();

        fn into(s: x86::state::Segment) -> state::Segment {
            let mut sg = state::Segment::default();

            sg.base = s.base;
            sg.limit = s.limit;
            sg.present = s.present;
            sg.selector = s.selector;
            sg.user_system = s.user_system;
            sg.code_data = s.code_data;
            sg.avl = s.available;
            sg.direction_conforming = s.direction_conforming;
            sg.gran = s.granularity;
            sg.long = s.long;
            sg.priv_level = s.dpl;
            sg.unusable = false;

            sg
        }

        sregs.cs = into(state.cs);
        sregs.ds = into(state.ds);
        sregs.ss = into(state.ss);
        sregs.es = into(state.es);
        sregs.fs = into(state.fs);
        sregs.gs = into(state.gs);

        // TODO: finish.

        unsafe { kvm::ioctl::set_sregs(self.fd(), &mut sregs)? };

        Ok(())
    }

    fn get_sregs(&self, state: &mut State) -> Result<()> {
        use self::kvm::structs::state;

        let sregs = {
            let mut sregs = state::SpecialRegisters::default();

            unsafe { kvm::ioctl::get_sregs(self.fd(), &mut sregs)? };

            sregs
        };

        fn into(sg: state::Segment) -> x86::state::Segment {
            let mut s = x86::state::Segment::default();

            s.base = sg.base;
            s.limit = sg.limit;
            s.present = sg.present;
            s.selector = sg.selector;
            s.user_system = sg.user_system;
            s.code_data = sg.code_data;
            s.available = sg.avl;
            s.direction_conforming = sg.direction_conforming;
            s.granularity = sg.gran;
            s.long = sg.long;
            s.dpl = sg.priv_level;

            s
        }

        state.cs = into(sregs.cs);
        state.ds = into(sregs.ds);
        state.ss = into(sregs.ss);
        state.es = into(sregs.es);
        state.fs = into(sregs.fs);
        state.gs = into(sregs.gs);

        // TODO: finish.

        Ok(())
    }
}

impl<'a> accel::VirtualCPU<'a> for VirtualCPU<'a> {
    fn sync(&self, state: &mut State, set: bool) -> Result<()> {
        if set {
            self.set_regs(state)?;
            self.set_sregs(state)?;
        } else {
            self.get_regs(state)?;
            self.get_sregs(state)?;
        }

        Ok(())
    }

    fn run(&self) -> Result<accel::ExitState> {
        let result = unsafe { kvm::ioctl::run(self.fd(), 0)? };

        // Compare with EINTR.
        if result == 4 {
            // TODO: handle pending unmasked signal.
            unimplemented!("Received interrupt while running");
        }

        let run = self.run_state();

        use self::run::ExitReason as ER;
        use accel::ExitState as ES;
        let state = match run.exit_reason {
            ER::Unknown => {
                let hw_exit_reason = unsafe { run.exit.unknown };
                ES::Unknown(hw_exit_reason)
            }
            ER::FailEntry => {
                let exit_reason = unsafe { run.exit.fail_entry };
                panic!(
                    "vCPU failed to enter: {:?} ({})",
                    exit_reason,
                    exit_reason as u32
                );
            }
            ER::InternalError => {
                let suberror = unsafe { run.exit.internal };
                panic!("Internal KVM error: {:?} ({})", suberror, suberror as u32)
            }
            _ => {
                let er = run.exit_reason;
                panic!("Unknown KVM exit reason: {:?} ({})", er, er as u32)
            }
        };

        Ok(state)
    }
}
