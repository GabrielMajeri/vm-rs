//! Structures used to control a virtual CPU's execution.

use std::sync::atomic::AtomicBool;

/// Can be obtained by memory mapping a virtual CPU file descriptor.
///
/// This structure is used both for reading state and controlling the CPU's execution.
#[repr(C)]
pub struct RunState {
    /// Request KVM to stop running when possible in order to
    /// inject external interrupts into the guest machine.
    pub request_interrupt_window: AtomicBool,

    /// This is polled by KVM right after starting to run. If set,
    /// KVM will exit immediately.
    ///
    /// This is useful for use in a signal handler.
    pub immediate_exit: AtomicBool,

    _padding1: [u8; 6],

    /// Reason for stopping the execution of the vCPU.
    pub exit_reason: ExitReason,

    /// True if requesting a window for interrupt injection succeeded.
    pub ready_for_interrupt_injection: bool,

    /// Value of interrupt flag.
    ///
    /// Only valid if in-kernel LAPIC is not used.
    pub if_flag: u8,

    /// Machine-specific flags.
    pub flags: RunFlags,

    /// Value of the CR8 register.
    ///
    /// Only valid if in-kernel LAPIC is not used.
    pub cr8: u64,

    /// Value of APIC base MSR.
    ///
    /// Only valid if in-kernel LAPIC is not used.
    pub apic_base: u64,

    /// A union containing additional data relating to the vCPU's exit.
    ///
    /// Valid fields depend on the `exit_reason`.
    pub exit: ExitData,
}

bitflags! {
    pub struct RunFlags: u16 {
        /// Set if vCPU is in system management mode.
        const X86_SMM = 1;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ExitReason {
    /// Hardware exit reason is valid.
    Unknown = 0,
    /// Port I/O emulation.
    Io = 2,
    Debug = 4,
    Hlt = 5,
    Mmio = 6,
    IrqWindowOpen = 7,
    Shutdown = 8,
    FailEntry = 9,
    Interrupt = 10,
    // TODO: is this actually used anywhere?
    SetTpr = 11,
    TprAccess = 12,
    Nmi = 16,
    /// An internal error occured in KVM.
    InternalError = 17,
}

/// Architecture-specific exit reason.
pub type HardwareExitReason = ::x86::vmx::ExitReason;

#[repr(C)]
pub union ExitData {
    /// The vCPU stopped running due to an unknown reason.
    pub unknown: HardwareExitReason,
    /// The vCPU failed to run.
    pub fail_entry: HardwareExitReason,
    /// The guest attempted to do port I/O.
    pub io: IoState,
    /// An internal kernel module error occured.
    pub internal: InternalError,
    _padding: [u8; 256],
}

/// KVM expects the application to `mmap` a region of memory,
/// starting at offset `data_offset`, of length `size * count`.
///
/// This memory region is a packed array of elemnts of size `size`,
/// and the application must either read from or write to this array.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IoState {
    /// True if we must write data, false if we must read data.
    pub direction: bool,
    /// Size in bytes of the I/O element.
    pub size: u8,
    /// The I/O port.
    pub port: u16,
    /// How many elements are in the array.
    pub count: u32,
    /// Offset used when `mmap`ing the file descriptor.
    pub data_offset: u64,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum InternalError {
    /// Emulating an instruction failed.
    ///
    /// On x86, this could mean the host does not support
    /// Second-Level Address Translation.
    Emulation = 1,
    /// An exception occured while handling an exception.
    SimultaneousExceptions = 2,
    /// The VM exited during event delivery.
    EventDelivery = 3,
}
