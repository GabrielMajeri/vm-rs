use super::{Handle, Result, ResultExt, io_codes};
use super::version;

/// A connection to the HAX kernel-mode driver.
pub struct Connection {
	handle: Handle
}

impl Connection {
	/// Creates a new connection to the HAX manager.
	// TODO: take a struct that describes how much memory has been requested.
	pub fn new() -> Result<Self> {
		let device = {
			const HAX_DEVICE_PATH: &str = r"\\.\HAX";

			let handle = Handle::new(HAX_DEVICE_PATH)
				.chain_err(|| "HAXM is not installed")?;

			Connection { handle }
		};

		device.check_capabilities()?;
		device.check_version()?;

		Ok(device)
	}

	pub (in super) fn create_vm(&self) -> Result<i32> {
		self.handle.receive::<i32>(io_codes::CREATE_VM)
	}

	fn check_capabilities(&self) -> Result<()> {
		let caps = self.handle.receive::<CapabilityInfo>(io_codes::GET_CAPABILITIES)?;

		info!(target: "hax", "HAX driver working: {}", caps.working());

		if !caps.working() {
			if !caps.vt_x_enabled() {
				bail!("Virtual Machine Extensions are not enabled, required by HAXM")
			}
			if !caps.nx_enabled() {
				bail!("The No eXecute feature is not enabled, required by HAXM")
			}
			if !caps.ug_enabled() {
				bail!("Unrestricted Guest is not supported by your CPU, required by HAXM")
			}
		}

		if let Some(memory_limit) = caps.memory_limit() {
			// TODO: check the requested memory is below the memory limit.
			assert!(memory_limit > 512 * 1024 * 1024, "Not enough memory allocated to HAXM")
		}

		Ok(())
	}

	fn check_version(&self) -> Result<()> {
		let version = self.handle.receive::<GetHaxVersion>(io_codes::GET_VERSION)?;

		info!(target: "hax", "HAX driver API version: minimum = {:?}, current = {:?}", version.compat_version, version.current_version);

		if version.too_old() {
			bail!("HAXM version is too old, please update your HAX driver");
		} else if version.too_new() {
			bail!("HAXM version is too new. Please update `vm-rs`")
		}

		Ok(())
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CapabilityInfo {
	// Bit 0: whether HAX is working.
	// Bit 1: whether there is a memory limit (`memory_quota` is valid).
	status: u16,
	// This field is valid when HAX is not working.
	// Bit 0: VT-x is not enabled.
	// Bit 1: NX is not enabled.
	// Bit 2: Whether the Unrestricted Guest feature is available.
	fail_info: u16,
	_padding: u32,
	memory_quota: u64
}

impl CapabilityInfo {
	fn working(&self) -> bool {
		(self.status & 1) != 0
	}

	fn memory_limit(&self) -> Option<usize> {
		if (self.status & 2) != 0 {
			Some(self.memory_quota as usize)
		} else {
			None
		}
	}

	fn vt_x_enabled(&self) -> bool {
		if !self.working() {
			(self.fail_info & 1) == 0
		} else {
			true
		}
	}

	fn nx_enabled(&self) -> bool {
		if !self.working() {
			(self.fail_info & 2) == 0
		} else {
			true
		}
	}

	fn ug_enabled(&self) -> bool {
		if !self.working() {
			(self.fail_info & 4) != 0
		} else {
			true
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
struct GetHaxVersion {
	// Minimum API version we have to implement.
	compat_version: version::HaxVersion,
	// The version of the HAX kernel driver.
	current_version: version::HaxVersion
}

impl GetHaxVersion {
	/// Checks whether the HAX module's version is too old.
	pub fn too_old(&self) -> bool {
		self.current_version < version::HAX_MINIMUM_VERSION
	}

	/// Checks whether the HAX module's version is too new.
	pub fn too_new(&self) -> bool {
		version::HAX_CURRENT_VERSION < self.compat_version
	}
}
