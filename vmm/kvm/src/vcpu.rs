use {VirtualMachine, RawFd, KVM_IO};
use errors::Result;
use object::Object;

use nix::unistd::close;
use nix::sys::mman::{mmap, munmap, PROT_READ, PROT_WRITE, MAP_SHARED};

use std::{mem, ptr};
use vm_x86::state::State;

/// Represents a virtual CPU of the virtual machine.
///
/// Each vCPU owns and runs on an associated thread.
#[derive(Debug)]
pub struct VirtualCPU<'vm> {
    vm: &'vm VirtualMachine<'vm>,
    fd: RawFd,
    run: &'static mut RunState,
}

#[repr(C)]
#[derive(Debug)]
struct RunState {}

impl<'a> VirtualCPU<'a> {
    pub(crate) fn new(vm: &'a VirtualMachine, fd: RawFd) -> Result<Self> {
        let vcpu = {
            let run = unsafe {
                let addr = ptr::null_mut();
                let size = vm.device().vcpu_mmap_size()?;
                let prot = PROT_READ | PROT_WRITE;
                let flags = MAP_SHARED;
                let offset = 0;

                let run_ptr = mmap(addr, size, prot, flags, fd, offset)?;

                mem::transmute(run_ptr)
            };

            VirtualCPU { vm, fd, run }
        };

        Ok(vcpu)
    }

    /// Synchronises the state of the kernel-mode virtual CPU registers
    /// with the user-space ones.
    ///
    /// If `set` is true, then the vCPUs registers will be updated,
    /// if false the `state` parameter will be updated from the vCPU registers.
    pub fn sync_state(&self, state: &mut State, set: bool) -> Result<()> {
        #[repr(C)]
        pub struct Registers {
            rax: u64,
            rbx: u64,
            rcx: u64,
            rdx: u64,
            rsi: u64,
            rdi: u64,
            rsp: u64,
            rbp: u64,
            r: [u64; 8],
            rip: u64,
            rflags: u64,
        }

        if set {
            let regs = {
                let mut r: Registers = unsafe { mem::uninitialized() };

                r.rax = state.r[0];
                r.rcx = state.r[1];
                r.rdx = state.r[2];
                r.rbx = state.r[3];

                r.rsp = state.r[4];
                r.rbp = state.r[5];

                r.rsi = state.r[6];
                r.rdi = state.r[7];

                unsafe {
                    let src = state.r.as_ptr().offset(8);
                    let dest = r.r.as_mut_ptr();
                    let len = r.r.len();
                    ptr::copy_nonoverlapping(src, dest, len);
                }

                r
            };

            unsafe {
                ioctl!(write_ptr set_regs with KVM_IO, 0x82; Registers);

                set_regs(self.fd, &regs)?;
            }
        } else {
            let r = unsafe {
                let mut regs = mem::uninitialized();

                ioctl!(read get_regs with KVM_IO, 0x81; Registers);

                get_regs(self.fd, &mut regs)?;

                regs
            };

            state.r[0] = r.rax;
            state.r[1] = r.rcx;
            state.r[2] = r.rdx;
            state.r[3] = r.rbx;

            state.r[4] = r.rsp;
            state.r[5] = r.rbp;

            state.r[6] = r.rsi;
            state.r[7] = r.rdi;

            unsafe {
                let src = r.r.as_ptr();
                let dest = state.r.as_mut_ptr().offset(8);
                let len = r.r.len();
                ptr::copy_nonoverlapping(src, dest, len);
            }

            state.ip = r.rip;
            state.flags = r.rflags;
        }

        Ok(())
    }
}

impl<'a> Drop for VirtualCPU<'a> {
    fn drop(&mut self) {
        unsafe {
            let run_ptr = mem::transmute::<&_, *mut _>(self.run);
            let size = mem::size_of::<RunState>();
            munmap(run_ptr, size).unwrap();
        }

        close(self.fd).unwrap();
    }
}

impl<'a> Object for VirtualCPU<'a> {
    fn fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests;

    // Checks that setting / reading general-purpose registers works.
    #[test]
    fn general_registers() {
        let vcpu = tests::get_vcpu();

        const RAX: u64 = 0xAB_CD;
        const RDX: u64 = 0x12_34;

        {
            let mut state: State = unsafe { mem::zeroed() };

            state.r[0] = RAX;
            state.r[2] = RDX;

            vcpu.sync_state(&mut state, true).unwrap();
        }

        {
            let state = {
                let mut state = unsafe { mem::uninitialized() };

                vcpu.sync_state(&mut state, false).unwrap();

                state
            };

            assert_eq!(state.r[0], RAX);
            assert_eq!(state.r[2], RDX);
        }
    }
}
