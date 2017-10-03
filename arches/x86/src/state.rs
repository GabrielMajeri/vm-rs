/// Stores information about a memory segment.
#[derive(Debug, Copy, Clone)]
pub struct Segment {
	/// The starting physical address of this segment,
	pub base: u64,
	/// The size of the segment. [base, base + limit) must be a valid memory range.
	pub limit: u32,
	/// The selector of this segment. Equal to the index in the GDT.
	pub selector: u16,
	/// Bits:
	/// - 0: Accessed
	/// - 1: Writable (for data) / Readable (for code)
	/// - 2: Direction (for data) / Conforming (for code)
	/// - 3: True for code, false for data.
	/// - 4: True for user descriptors, false for system descriptors.
	/// - 5-6: Descriptor Privilege Level.
	/// - 7: Is present.
	/// - 8-11: Limit (unused).
	/// - 12: available for OS to use.
	/// - 13: Is in long mode.
	/// - 14: Operand size (16 bit / 32 bit).
	/// - 15: Granularity (byte / 4-KiB).
	pub flags: u16
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
	///
	/// Bits:
	/// - 0: carry flag.
	/// - 1: reserved, always 1.
	/// - 2: parity flag.
	/// - 3: reserved.
	/// - 4: adjust flag.
	/// - 5: reserved.
	/// - 6: zero flag.
	/// - 7: sign flag.
	/// - 8: trap flag.
	/// - 9: interrupt enable flag.
	/// - 10: direction flag.
	/// - 11: overflow flag.
	/// - 12-13: I/O privilege level.
	/// - 14: nested task flag.
	/// - 15: reserved.
	/// - 16: resume flag.
	/// - 17: virtual 8086 flag.
	/// - 18: alignment check.
	/// - 19: virtual interrupt flag.
	/// - 20: virtual interrupt pending.
	/// - 21: able to use CPUID instruction.
	pub flags: u64,
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
	///
	/// Bits:
	/// - 0: protected mode enable.
	/// TODO: finish
	pub cr0: u64
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
		let cs = Segment {
			base: 0xFFFF_0000,
			selector: 0xF000,
			limit: 0xFFFF,
			// Accessed, readable, user, present.
			flags: 0b1001_0011
		};

		let ds = Segment {
			base: 0,
			selector: 0,
			limit: 0xFFFF,
			// Accessed, readable, user, present.
			flags: 0b1001_0011
		};

		State {
			r,
			ip,
			cs,
			ds,
			ss: ds,
			es: ds,
			fs: ds,
			gs: ds,
			flags: 0b10,
			// The CD and NW flags and bit 4 are set.
			cr0: (1 << 30) | (1 << 29) | (1 << 4),
		}
	}
}

/// Contains all the architectural MSRs.
#[derive(Debug, Copy, Clone)]
pub struct MSRState {
	/// The time-stamp counter.
	pub tsc: u64,
	/// The SYSENTER code segment.
	pub sysenter_cs: u64
}

/// Contains the x87 FPU and SSE state.
#[derive(Debug, Copy, Clone)]
pub struct FPUState {
	/// The status word of the FPU.
	// TODO: use a bitfield
	pub status: u16
}

impl Default for FPUState {
	fn default() -> Self {
		FPUState {
			status: 0
		}
	}
}
