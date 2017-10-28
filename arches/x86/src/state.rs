//! Structures representing the x86 processor state.

use fpu;

/// Stores information about a memory segment.
#[derive(Debug, Default, Copy, Clone)]
pub struct Segment {
    /// The starting physical address of this segment,
    pub base: u64,
    /// The size of the segment. [base, base + limit) must be a valid memory range.
    pub limit: u32,
    /// The selector of this segment. Equal to the index in the GDT.
    pub selector: u16,
    /// True if segment is present.
    pub present: bool,
    /// True if user segment, false if system segment.
    pub user_system: bool,
    /// True for code, false for data.
    pub code_data: bool,
    /// Writable (for data) / readable (for code).
    pub write_read: bool,
    /// Direction (for data) / Conforming (for code)
    pub direction_conforming: bool,
    /// Descriptor Privilege Level.
    pub dpl: u8,
    /// True if this is a long mode segment.
    pub long: bool,
    /// Operand size (16 bit / 32 bit).
    ///
    /// Must be 0 in 64-bit mode.
    pub op_size: bool,
    /// Granularity (byte / 4-KiB).
    pub granularity: bool,
    /// Set by the CPU when it is accessed.
    pub accessed: bool,
    /// Available for OS to use.
    pub available: bool,
}

/// Stores the state for an `x86_64` CPU.
#[derive(Debug, Copy, Clone)]
pub struct State {
    /// This is a short list of the general-purpose registers.
    ///
    /// Some of these "general-purpose" register actually have a use in various function calls.
    /// Format: R# = <name> (<mnemonic>)
    /// - R0 = Accumulator (RAX)
    /// - R1 = Counter (RCX)
    /// - R2 = Data (RDX)
    /// - R3 = Base (RBX)
    /// - R4 = Stack Pointer (SP)
    /// - R5 = Base Pointer (BP)
    /// - R6 = Source Index (SI)
    /// - R7 = Destination Index (DI)
    /// - R8 through R15 - General-purpose
    pub r: [u64; 16],
    /// The instruction pointer stores the address of the current instruction.
    pub ip: u64,
    /// The FLAGS register.
    pub flags: Flags,
    /// The current code segment, a segment of memory which contains executable code.
    pub cs: Segment,
    /// The current data segment.
    pub ds: Segment,
    /// The stack segment.
    pub ss: Segment,
    /// Extra segment #1.
    pub es: Segment,
    /// Extra segment #2.
    pub fs: Segment,
    /// Extra segment #3.
    pub gs: Segment,
    /// Control register 0.
    pub cr0: Cr0,
    /// Contains the linear address of the last page-fault.
    pub cr2: u64,
    /// Contains the physical address of the top-level page mapping table.
    pub cr3: u64,
    /// Control register 4.
    pub cr4: Cr4,
    /// Task-priority register.
    pub cr8: u64,
    /// This register is architectural on AMD64.
    pub efer: Efer,

    /// The x87 floating-point unit's state.
    pub fpu: fpu::X87State,
    /// The SSE / AVX registers.
    pub sse: fpu::SseState,
}

impl Default for State {
    fn default() -> Self {
        // The `default` state is the state of the processor after startup.
        // See Intel Architecture Manual, Vol. 3A, Section "9.1.1 Processor State After Reset"

        let mut r = [0; 16];

        // TODO: this is supposed to be 0xn0600, where n is extended model value (what is that?).
        // For now, it is 1.
        r[2] = 0x1_0600;

        // We must start at 16 bytes before 4 GiB.
        let ip = 0xFFF0;

        let common = Segment {
            base: 0,
            selector: 0,
            limit: 0xFFFF,
            present: true,
            user_system: true,
            code_data: true,
            ..Segment::default()
        };

        let cs = Segment {
            base: 0xFFFF_0000,
            selector: 0xF000,
            ..common
        };

        let ds = common;

        State {
            r,
            ip,
            cs,
            ds,
            ss: ds,
            es: ds,
            fs: ds,
            gs: ds,
            flags: Flags::default(),
            cr0: Cr0::default(),
            cr2: 0,
            cr3: 0,
            cr4: Cr4::default(),
            cr8: 0,
            efer: Efer::default(),
            fpu: fpu::X87State::default(),
            sse: fpu::SseState::default(),
        }
    }
}

bitflags! {
    /// The FLAGS register contains flags set by various operations,
    /// as well as some useful flags which change the behaviour of
    /// basic operations.
    ///
    /// All other bits are reserved and must be preserved.
    pub struct Flags: u64 {
        /// Carry flag, set if last operation resulted in a carry.
        const CARRY = 1 << 0;

        /// Reserved, must always be set.
        const RESERVED_ONE = 1 << 1;

        /// Parity of last result.
        const PARITY = 1 << 2;

        /// Adjust flag. Used for binary-coded decimals.
        const ADJUST = 1 << 4;

        /// Last value was 0.
        const ZERO = 1 << 6;

        /// Sign of last result.
        const SIGN = 1 << 7;

        /// If set, stops execution after every instruction,
        /// for debugging purposes.
        const TRAP = 1 << 8;

        /// Enables or disables extern interrupts.
        ///
        /// Note that exceptions are NMIs are not affected.
        const INTERRUPT = 1 << 9;

        /// If set, direction of `rep` instructions is backwards.
        const DIRECTION = 1 << 10;

        /// Set if an operation resulted in an overflow.
        const OVERFLOW = 1 << 11;

        /// Privilege level required for port I/O operations.
        const IOPRIV = 0b11 << 12;

        /// Indicates that a system task invoked another
        /// using a CALL instruction.
        const NESTED_TASK = 1 << 14;

        /// Used to temporarily disable debug exceptions.
        const RESUME = 1 << 16;

        /// Virtual 8086 mode.
        const VIRTUAL_8086 = 1 << 17;

        /// When set, all memory accesses must be aligned.
        const ALIGNMENT_CHECK = 1 << 18;

        /// Enables virtual interrupts.
        const VIRTUAL_INTERRUPT = 1 << 19;

        /// A virtual interrupt is pending.
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;

        /// CPU supports CPUID instruction.
        const CPUID = 1 << 21;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::RESERVED_ONE
    }
}

bitflags! {
    /// Control register 0 contains basic flags for modifying
    /// the way the CPU runs.
    pub struct Cr0: u64 {
        /// When set, processor is in protected mode.
        const PROTECTED_MODE = 1 << 0;
        /// Controls the WAIT / FWAIT instructions.
        const MONITOR_COPROCESSOR = 1 << 1;
        /// If clear, x87 FPU is present.
        const EMULATION = 1 << 2;
        /// Allows saving x87 task context.
        const TASK_SWITCHED = 1 << 3;
        /// Whether external math coprocessor is 80287 (unset) or 80387 (set).
        const EXTENSION_TYPE = 1 << 4;
        /// Enables internal x87 floating point error reporting when set.
        const NUMERIC_ERROR = 1 << 5;
        /// When set, prevents CPU in ring 0 from writing to read-only pages.
        const WRITE_PROTECT = 1 << 16;
        /// Required for alignment check.
        const ALIGNMENT_MASK = 1 << 18;
        /// If set, disables write-through caching.
        const NOT_WRITE_THROUGH = 1 << 29;
        /// If set, disables memory caching.
        const CACHE_DISABLE = 1 << 30;
        /// If set, enables paging.
        const PAGING = 1 << 31;
    }
}

impl Default for Cr0 {
    fn default() -> Self {
        Cr0::EXTENSION_TYPE | Cr0::NOT_WRITE_THROUGH | Cr0::CACHE_DISABLE
    }
}

bitflags! {
    /// CR4 is used to control additional processor features.
    pub struct Cr4: u64 {
        /// Enables virtual 8086 mode extensions.
        const VIRTUAL_8086 = 1 << 0;
        /// Enable protected-mode virtual interrupts.
        const VIRTUAL_INTERRUPTS = 1 << 1;
        /// If set, RDTSC can only be used in ring 0.
        const TIME_STAMP_DISABLE = 1 << 2;
        /// Enables debug register based breaks on I/O access.
        const DEBUGGING_EXTENSIONS = 1 << 3;
        /// If set, page size is increased to 4 MiB.
        ///
        /// This is ignored when using PAE or in long mode.
        const PAGE_SIZE_EXTENSION = 1 << 4;
        /// If set, 32-bit virtual addresses are translated into 36-bit physical addresses.
        const PHYSICAL_ADDRESS_EXTENSION = 1 << 5;
        /// If set, enables machine check interrupts to occur.
        const MACHINE_CHECK_EXCEPTION = 1 << 6;
        /// Enables global pages.
        const PAGE_GLOBAL_ENABLE = 1 << 7;
        /// If set, RDPMC may be used at any privilege level.
        const PERF_MONITOR_ENABLE = 1 << 8;
        /// If set, enables process-context identifiers.
        const PROCESS_CONTEXT_ID = 1 << 17;
        /// If set, enables SSE and fast FPU state save & restore.
        const OS_FXSR = 1 << 9;
        /// Support for unmasked SIMD floating-point exceptions.
        const OS_XMM_EXCEPTIONS = 1 << 10;
        /// Enables Intel VT-x.
        const VIRTUAL_MACHINE_EXTENSIONS = 1 << 13;
        /// Enable Trusted Execution Technology.
        const SAFER_MODE_EXTENSIONS = 1 << 14;
        /// Enables the RD/WR FS/GS BASE instructions.
        const FS_GS_BASE = 1 << 16;
        /// Support for the XSAVE and Processor Extended States.
        const OS_XSAVE = 1 << 18;
        /// Execution of code in a higher ring generates a fault.
        const SM_EXEC_PREVENTION = 1 << 20;
        /// Accessing data in a higher ring generates a fault.
        const SM_ACCESS_PREVENTION = 1 << 21;
        /// Enables protection keys.
        const PROTECTION_KEY_ENABLE = 1 << 22;
    }
}

impl Default for Cr4 {
    fn default() -> Self {
        Cr4::empty()
    }
}

bitflags! {
    /// Extended Feature Enable Register.
    pub struct Efer: u64 {
        /// Enables SYSCALL.
        const SYSTEM_CALL = 1 << 0;
        /// Enables long mode if set.
        const LM_ENABLE = 1 << 8;
        /// True if long mode is active.
        const LM_ACTIVE = 1 << 10;
        /// Enables No eXecute / eXecute Disable.
        const NX_ENABLE = 1 << 11;
        /// Enables Secure Virtual Machine.
        const SVM_ENABLE = 1 << 12;
        /// Enables long mode segment limit.
        const LMSL_ENABLE = 1 << 13;
        /// Fast FXSAVE / FXRSTOR.
        const FAST_FXSR = 1 << 14;
        /// Translation cache extension.
        const TRANSLATION_CACHE = 1 << 15;
    }
}

impl Default for Efer {
    fn default() -> Self {
        Efer::empty()
    }
}
