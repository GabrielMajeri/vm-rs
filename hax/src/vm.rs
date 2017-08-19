use super::{Handle, Connection, Result, io_codes, version};
use common::{MemoryBlock, MemoryRegion};
use std::marker;

/// A virtual machine is assigned some memory and one or more virtual CPUs.
pub struct VirtualMachine<'conn> {
	handle: Handle,
	id: i32,
	phantom: marker::PhantomData<&'conn Connection>
}

impl<'conn> VirtualMachine<'conn> {
	/// Creates a new virtual machine, with no vCPUs and no memory allocated.
	pub fn new(conn: &'conn Connection) -> Result<Self> {
		let id = conn.create_vm()?;

		info!(target: "hax", "Creating VM #{}", id);

		let vm = VirtualMachine {
			handle: Handle::new(get_hax_vm_path(id))?,
			id,
			phantom: marker::PhantomData
		};

		vm.notify_version()?;

		Ok(vm)
	}

	/// Gives a block of memory to this VM.
	///
	/// Note: memory blocks cannot be reclaimed while the VM is alive.
	/// The memory will be freed when the VM is destroyed.
	pub fn allocate_memory(&self, block: MemoryBlock) -> Result<()> {
		// TODO: support allocating in smaller blocks.
		assert!(block.size() <= ::std::u32::MAX as usize, "Memory block is too big");

		#[repr(C)]
		struct AllocateMemoryInfo {
			size: u32,
			_padding: u32,
			address: u64
		}

		let size = block.size() as u32;
		let address = block.into_raw() as u64;

		let alloc_info = AllocateMemoryInfo {
			size,
			_padding: 0,
			address
		};

		self.handle.send(io_codes::ALLOC_RAM, alloc_info)
	}

	/// Maps an allocated host memory block to a guest physical block.
	pub fn map_memory(&self, region: MemoryRegion, flags: MemoryRegionFlags) -> Result<()> {
		// TODO: support setting memory properties for more than 4 GiBs.
		assert!(region.size() <= ::std::u32::MAX as usize, "Memory region size is too big");

		#[repr(C)]
		struct SetMemoryInfo {
			physical_addr: u64,
			size: u32,
			flags: u8,
			_padding: [u8; 3],
			virtual_addr: u64
		}

		let physical_addr = region.guest_address() as u64;
		let size = region.size() as u32;
		let flags = flags.bits();
		let virtual_addr = region.host_address() as u64;

		let set_mem_info = SetMemoryInfo {
			physical_addr,
			size,
			flags,
			_padding: [0; 3],
			virtual_addr
		};

		self.handle.send(io_codes::SET_RAM_FLAGS, set_mem_info)
	}

	pub (in super) fn id(&self) -> i32 {
		self.id
	}

	pub (in super) fn create_vcpu(&self, id: i32) -> Result<()> {
		self.handle.send(io_codes::CREATE_VCPU, id)
	}

	fn notify_version(&self) -> Result<()> {
		self.handle.send(io_codes::NOTIFY_VERSION, NotifyVersion::new())?;
		Ok(())
	}
}

bitflags! {
	/// Describes the characteristics of a memory region on the guest machine.
	pub struct MemoryRegionFlags: u8 {
		/// Can only be read from.
		const READ_ONLY = 0x1;

		/// Unmapped, used for Memory Mapped IO or similar.
		const INVALID = 0x80;
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NotifyVersion {
	// Current API version used by `vm-rs`.
	current: version::HaxVersion,
	// Minimum API version supported.
	minimum: version::HaxVersion
}

impl NotifyVersion {
	fn new() -> Self {
		NotifyVersion {
			current: version::HAX_CURRENT_VERSION,
			minimum: version::HAX_MINIMUM_VERSION
		}
	}
}

fn get_hax_vm_path(id: i32) -> String {
	format!(r"\\.\hax_vm{:02}", id)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn vm_path() {
		let path = get_hax_vm_path(12);
		assert_eq!(path, r"\\.\hax_vm12");
	}
}
