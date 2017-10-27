//! Structures representing interrupt controllers' state.

/// Structure containing the data of an IRQ chip.
///
/// You must fill the `ChipId` field with
/// the chip whose state you want to retrieve.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqChip {
    pub id: ChipId,
    _padding: u32,
    pub state: IrqChipState,
}

/// The type of the IRQ chip.
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ChipId {
    PIC1 = 0,
    PIC2 = 1,
    IOAPIC = 2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union IrqChipState {
    pub pic_state: PicState,
    pub ioapic_state: IoApicState,
    _padding: [u8; 512],
}

/// The state of an emulated Programmable Interrupt Controller.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PicState {
    /// Last IRR value for edge detection.
    pub last_irr: u8,
    /// Interrupt request register.
    pub irr: u8,
    /// Interrupt mask register.
    pub imr: u8,
    /// Interrupt service register.
    pub isr: u8,
    /// Highest IRQ priority.
    pub priority: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub poll: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_auto_eoi: u8,
    pub special_fully_nested_mode: u8,
    /// True if the guest initialized the PIC with 4 bytes.
    pub init4: bool,
    /// PIIX edge/trigger selection.
    pub elcr: u8,
    pub elcr_mask: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IoApicState {
    /// The APIC base physical address.
    pub base_address: u64,
    /// Value of the IOREGSEL register.
    pub reg_sel: u32,
    /// ID of the I/O APIC.
    pub id: u32,
    /// Interrupt request register.
    pub irr: u32,
    _padding: u32,
    /// Each IRQ is configured by one 64-bit register.
    // TODO: replace this with a bitfield.
    pub redir_tbl: [u64; 24],
}
