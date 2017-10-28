//! Structures representing the virtual CPU's state.

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Registers {
    /// RAX, RBX, RCX, RDX,
    /// RSI, RDI, RSP, RBP
    /// then R8 through R15.
    pub r: [u64; 16],
    /// Instruction pointer.
    pub ip: u64,
    /// FLAGS register.
    pub flags: u64,
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct SpecialRegisters {
    pub cs: Segment,
    pub ds: Segment,
    pub es: Segment,
    pub fs: Segment,
    pub gs: Segment,
    pub ss: Segment,

    pub tr: Segment,
    pub ldt: Segment,

    pub gdt: DescriptorTable,
    pub idt: DescriptorTable,

    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,

    /// Value of EFER MSR.
    pub efer: u64,
    /// Value of APIC BASE MSR.
    pub apic_base: u64,
    /// A bitmap of pending external interrupts.
    ///
    /// At most one bit may be set at a time.
    ///
    /// The interrupt has been acknowledged by the LAPIC
    /// but has not yet been injected into the CPU core.
    pub interrupt_bitmap: [u64; 4],
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Segment {
    /// Starting address of memory segment.
    pub base: u64,
    /// Memory limit.
    pub limit: u32,
    /// Index in the GDT.
    pub selector: u16,
    /// Type of selector.
    pub code_data: bool,
    /// Whether it is present or not.
    pub present: bool,
    /// Descriptor Privilege Level.
    pub priv_level: u8,
    /// Direction / conforming.
    pub direction_conforming: bool,
    /// User / system.
    pub user_system: bool,
    /// Long mode.
    pub long: bool,
    /// Granularity.
    pub gran: bool,
    /// Bit available for use by CPU.
    pub avl: bool,
    /// Bit reserved for the CPU's use.
    pub unusable: bool,
    padding: u8,
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct DescriptorTable {
    pub base: u64,
    pub limit: u16,
    _padding: [u16; 3],
}
