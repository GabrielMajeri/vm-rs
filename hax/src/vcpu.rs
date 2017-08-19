#![allow(dead_code)]

use {Handle, VirtualMachine, Result, io_codes};
use std::{marker, mem};
use x86::{state, vmx, msr};

/// Contains the state of a virtual CPU core.
///
/// This also contains FPU state (x87 and SSE) and the state of the MSRs.
pub struct VirtualCPU<'vm> {
	handle: Handle,
	tunnel: &'vm Tunnel,
	io_buffer: *const u8,
	phantom: marker::PhantomData<&'vm VirtualMachine<'vm>>
}

impl<'vm> VirtualCPU<'vm> {
	/// Creates a new virtual CPU for the given machine.
	// TODO: automatically allocate IDs.
	pub fn new(vm: &'vm VirtualMachine, id: u8) -> Result<Self> {
		let vcpu_id = id as i32;

		vm.create_vcpu(id as i32)?;

		let handle = Handle::new(get_hax_vcpu_path(vm.id(), vcpu_id))?;

		#[repr(C)]
		#[derive(Copy, Clone)]
		struct TunnelInfo {
			tunnel_addr: u64,
			io_buffer_addr: u64,
			size: u16,
			_padding: [u16; 3]
		}

		let tunnel_info = handle.receive::<TunnelInfo>(io_codes::SET_UP_TUNNEL)?;

		info!(target: "hax", "Created HAX tunnel, structure size: {} bytes", tunnel_info.size);

		if (tunnel_info.size as usize) < mem::size_of::<Tunnel>() {
			bail!("Unsupported HAX tunnel size");
		}

		assert_ne!(tunnel_info.tunnel_addr, 0, "Received null pointer for HAX tunnel");

		assert_ne!(tunnel_info.io_buffer_addr, 0, "Received null pointer for HAX I/O buffer");

		let tunnel = unsafe { mem::transmute(tunnel_info.tunnel_addr as usize) };
		let io_buffer = unsafe { mem::transmute(tunnel_info.io_buffer_addr as usize) };

		let vcpu = VirtualCPU {
			handle,
			tunnel,
			io_buffer,
			phantom: marker::PhantomData
		};

		Ok(vcpu)
	}

	/// Runs the virtual CPU.
	pub fn run(&self) -> Result<RunResult> {
		self.handle.send(io_codes::RUN, ())?;

		let run_result = RunResult {
			exit_status: self.tunnel.status,
			exit_reason: self.tunnel.exit_reason
		};

		Ok(run_result)
	}

	/// Retrieves the state of the virtual CPU.
	pub fn get_cpu_state(&self, state: &mut state::State) -> Result<()> {
		let hax_state: State = self.handle.receive(io_codes::GET_VCPU_STATE)?;

		*state = state::State {
			r: hax_state.r,
			ip: hax_state.ip,
			flags: hax_state.flags,
			cs: hax_state.cs.to_segment(),
			ds: hax_state.ds.to_segment(),
			ss: hax_state.ss.to_segment(),
			es: hax_state.es.to_segment(),
			fs: hax_state.fs.to_segment(),
			gs: hax_state.gs.to_segment(),
			cr0: hax_state.cr0,
		};

		Ok(())
	}

	/// Sets the state of the virtual CPU.
	pub fn set_cpu_state(&self, state: &state::State) -> Result<()> {
		let hax_state = State {
			r: state.r,
			ip: state.ip,
			flags: state.flags,
			cs: SegmentDescriptor::from(state.cs),
			ds: SegmentDescriptor::from(state.ds),
			ss: SegmentDescriptor::from(state.ss),
			es: SegmentDescriptor::from(state.es),
			fs: SegmentDescriptor::from(state.fs),
			gs: SegmentDescriptor::from(state.gs),
			cr0: state.cr0,
			..unsafe { mem::zeroed() }
		};

		self.handle.send(io_codes::SET_VCPU_STATE, hax_state)
	}

	/// Gets the state of the CPU's MSRs.
	pub fn get_msr_state(&self, msrs: &mut state::MSRState) -> Result<()> {
		let msr_data = MSRData::from(msrs);

		let msr_data: MSRData = self.handle.transmit(io_codes::GET_MSR_STATE, msr_data)?;

		msr_data.into(msrs);

		Ok(())
	}

	/// Sets the MSR state.
	pub fn set_msr_state(&self, msrs: &state::MSRState) -> Result<()> {
		let msr_data = MSRData::from(msrs);

		// The return value is unused but HAX still requires it to exist.
		let msr_data: MSRData = self.handle.transmit(io_codes::SET_MSR_STATE, msr_data)?;

		assert_eq!(msr_data.done, msr_data.count, "HAX did not update some MSRs");

		Ok(())
	}

	/// Gets the state of the FPU.
	pub fn get_fpu_state(&self, state: &mut state::FPUState) -> Result<()> {
		let hax_state: FPUState = self.handle.receive(io_codes::GET_FPU_STATE)?;

		*state = state::FPUState {
			status: hax_state.status
		};

		Ok(())
	}

	/// Sets the state of the FPU.
	pub fn set_fpu_state(&self, state: &state::FPUState) -> Result<()> {
		// TODO: finish
		let hax_state = FPUState {
			status: state.status,
			..unsafe { mem::zeroed() }
		};

		self.handle.send(io_codes::SET_FPU_STATE, hax_state)
	}

	/// Generates an interrupt.
	pub fn interrupt(&self, vector: u8) -> Result<()> {
		self.handle.send(io_codes::INJECT_INTERRUPT, vector)
	}
}

fn get_hax_vcpu_path(vm_id: i32, id: i32) -> String {
	format!(r"\\.\hax_vm{:02}_vcpu{:02}", vm_id, id)
}

/// The result of running a virtual CPU, containing information on why the execution stopped.
#[derive(Debug, Copy, Clone)]
pub struct RunResult {
	exit_status: ExitStatus,
	exit_reason: vmx::ExitReason
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Tunnel {
	exit_reason: vmx::ExitReason,
	flag: u32,
	status: ExitStatus,
	user_event_pending: u32,
	ready_for_interrupt_injection: i32,
	request_interrupt_window: i32,
	state: TunnelState
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum PIODirection {
	Read = 0,
	Write = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ProgrammedIo {
	direction: PIODirection,
	// TODO: document these fields??
	df: u8,
	size: u16,
	port: u16,
	count: u16,
	flags: u8,
	padding: [u8; 6],
	vaddr: u64
}

#[repr(C)]
#[derive(Copy, Clone)]
union TunnelState {
	pio: ProgrammedIo,
	// TODO: mmio??
	gla: u64
}

/// The reason why the virtual CPU stopped running.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ExitStatus {
	/// IO port request.
	IoRequest = 1,
	/// A MMIO instruction.
	Mmio = 2,
	/// We don't have unrestricted guest, so we have to emulate real mode.
	RealModeEmulation = 3,
	/// Interrupt window open.
	Interrupt = 4,
	/// Unknown reason for exit, possibly guest requesting shutdown.
	UnknownVMExit = 5,
	/// The HLT instruction has been used.
	Halt = 6,
	/// Reboot request, possibly a triple fault.
	StateChange = 7,
	/// Should only be paused when VCPU is destroyed.
	Paused = 8,
	/// Use the new fast MMIO mode.
	FastMmio = 9
}

/// This structures stores the state of a segment descriptor.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SegmentDescriptor {
	// Index of the segment descriptor in the GDT.
	selector: u16,
	_padding1: u16,
	// The size, in bytes, of the memory segment.
	limit: u32,
	// The start physical address.
	base: u64,
	// Segment descriptor flags.
	flags: u32,
	_padding2: u32,
}

impl SegmentDescriptor {
	fn from(seg: state::Segment) -> Self {
		SegmentDescriptor {
			selector: seg.selector,
			_padding1: 0,
			limit: seg.limit,
			base: seg.base,
			flags: seg.flags as u32,
			_padding2: 0
		}
	}

	fn to_segment(&self) -> state::Segment {
		state::Segment {
			selector: self.selector,
			limit: self.limit,
			base: self.base,
			flags: self.flags as u16
		}
	}
}

// TODO: document: what is this used for?
// Value as follows:
// - bit 0: STI blocking.
// - bit 1: movss blocking.
// - bit 2: SMI blocking.
// - bit 3: NMI blocking.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct InterruptibilityState(u32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct State {
	// Order: RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8 through R15.
	r: [u64; 16],

	ip: u64,

	flags: u64,

	cs: SegmentDescriptor,
	ss: SegmentDescriptor,
	ds: SegmentDescriptor,
	es: SegmentDescriptor,
	fs: SegmentDescriptor,
	gs: SegmentDescriptor,

	ldt: SegmentDescriptor,
	tr: SegmentDescriptor,

	gdt: SegmentDescriptor,
	idt: SegmentDescriptor,

	cr0: u64,
	cr2: u64,
	cr3: u64,
	cr4: u64,

	dr0: u64,
	dr1: u64,
	dr2: u64,
	dr3: u64,
	dr6: u64,
	dr7: u64,

	// TODO: unknown, what does this do?
	pde: u64,

	/// Extended features enable register.
	efer: u32,

	sysenter_cs: u32,
	sysenter_ip: u32,
	sysenter_sp: u32,

	// TODO: no idea what this is used for??
	activity_state: u32,

	_padding1: u32,

	// TODO: no idea either.
	interruptibility_state: InterruptibilityState,

	_padding2: u64
}

const MSR_MAX_COUNT: usize = 32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct MSR {
	address: u32,
	_padding: u32,
	value: u64
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct MSRData {
	// Number of MSRs in the `entries` array.
	count: u16,
	// Number of MSRs that HAX updated.
	// After transmitting `GET_MSR_STATE`, the received structure will have this many MSRs in it.
	done: u16,
	_padding: [u16; 2],
	entries: [MSR; MSR_MAX_COUNT]
}

impl MSRData {
	fn new() -> Self {
		unsafe { mem::zeroed() }
	}

	fn push(&mut self, address: u32, value: u64) {
		self.entries[self.count as usize] = MSR {
			address,
			_padding: 0,
			value
		};

		self.count += 1;
	}

	fn from(state: &state::MSRState) -> Self {
		let mut data = Self::new();

		data.push(msr::TSC, state.tsc);

		data.push(msr::SYSENTER_CS, state.sysenter_cs);
		//data.push(msr::SYSENTER_SP);
		//data.push(msr::SYSENTER_IP);

		// BUG: these are only available on x86_64 targets.
		//data.push(msr::EFER);
		//data.push(msr::STAR);
		//data.push(msr::LSTAR);
		//data.push(msr::CSTAR);
		//data.push(msr::FMASK);
		//data.push(msr::KERNELGSBASE);

		data
	}

	fn into(self, msrs: &mut state::MSRState) {
		let received_msrs = self.entries.split_at(self.done as usize).0;

		for msr in received_msrs {
			match msr.address {
				msr::TSC => msrs.tsc = msr.value,
				msr::SYSENTER_CS => msrs.sysenter_cs = msr.value,
				// TODO: same for others.
				_ => unreachable!("HAX returned unknown MSR")
			}
		}
	}
}

/// See the FXSAVE / FXSTOR assembly instructions for information on this table.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct FPUState {
	control: u16,
	status: u16,
	tag: u8,
	_padding1: u8,
	// First 11 bits are FPU op code.
	op: u16,
	// In 16-bit mode, the first 32 bits are the offset, next 16 are the segment.
	ip: u64,
	// In 16-bit mode, the first 32 bits are the offset, next 16 are the segment.
	data_ptr: u64,
	mxcsr: u32,
	mxcsr_mask: u32,
	// FPU stack, each entry is supposed to be 80-bit wide (rounded up to 128).
	st: [[u64; 2]; 8],
	// SSE stack.
	mmx: [[u64; 2]; 16],
	_padding2: [u64; 12]
}

#[repr(C)]
struct FastMMIO {
	gpa: u64,
	gpa2: u64,
	size: u8,
	direction: u8,
	reg_index: u16,
	_padding: u32,
	cr0: u64,
	cr2: u64,
	cr3: u64,
	cr4: u64
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn vcpu_path() {
		let path = get_hax_vcpu_path(12, 34);
		assert_eq!(path, r"\\.\hax_vm12_vcpu34");
	}
}
