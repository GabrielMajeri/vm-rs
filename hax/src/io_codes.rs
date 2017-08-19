/// Creates a device control code.
const fn create_io_code(function: u32) -> u32 {
	const HAX_DEVICE_TYPE: u32 = 0x4000;

	(HAX_DEVICE_TYPE << 16) | (function << 2)
}

// Called on `Connection`.
pub const GET_VERSION: u32 = create_io_code(0x900);
pub const GET_CAPABILITIES: u32 = create_io_code(0x910);
pub const CREATE_VM: u32 = create_io_code(0x901);

// Called on `VirtualMachine`.
pub const NOTIFY_VERSION: u32 = create_io_code(0x910);
pub const ALLOC_RAM: u32 = create_io_code(0x903);
pub const SET_RAM_FLAGS: u32 = create_io_code(0x904);
pub const CREATE_VCPU: u32 = create_io_code(0x902);

// Called on `VirtualCPU`.
pub const SET_UP_TUNNEL: u32 = create_io_code(0x90b);
pub const RUN: u32 = create_io_code(0x906);

pub const GET_VCPU_STATE: u32 = create_io_code(0x90e);
pub const SET_VCPU_STATE: u32 = create_io_code(0x90d);

pub const GET_MSR_STATE: u32 = create_io_code(0x908);
pub const SET_MSR_STATE: u32 = create_io_code(0x907);

pub const GET_FPU_STATE: u32 = create_io_code(0x90a);
pub const SET_FPU_STATE: u32 = create_io_code(0x909);

pub const INJECT_INTERRUPT: u32 = create_io_code(0x90c);
