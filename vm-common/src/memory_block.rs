use { HostAddress, MemoryRegion };
use std::{mem, io, slice};

/// A page-aligned memory block that can be used by a virtual machine.
pub struct MemoryBlock {
	address: HostAddress,
	size: usize
}

impl MemoryBlock {
	/// Allocates pages of memory.
	pub fn allocate(pages: usize) -> Result<Self, io::Error> {
		// Allocate a multiple of 4 KiB.
		// TODO: what if some host OSes don't use 4 KiB as their page size?
		let size = pages * 4096;

		extern "system" {
			fn VirtualAlloc(address: usize, size: usize, allocation_type: u32, protection: u32) -> usize;
		}

		const MEM_COMMIT: u32 = 0x1000;
		const MEM_RESERVE: u32 = 0x2000;
		// Since we don't actually "execute" code, we don't need it to also have execute flags.
		const PAGE_READWRITE: u32 = 0x4;

		let address = unsafe {
			VirtualAlloc(0, size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE)
		};

		if address == 0 {
			Err(io::Error::last_os_error())
		} else {
			let block = MemoryBlock {
				address,
				size
			};

			Ok(block)
		}
	}

	/// Returns the length of the memory block.
	pub fn size(&self) -> usize {
		self.size
	}

	/// Returns a mutable view into the memory block.
	pub fn as_raw_mut(&self) -> &mut [u8] {
		unsafe {
			slice::from_raw_parts_mut(
				mem::transmute(self.address),
				self.size
			)
		}
	}

	/// Turns the memory block into a raw pointer to the memory.
	pub fn into_raw(self) -> HostAddress {
		let memory = self.address;
		mem::forget(self);
		memory
	}

	/// Returns an unmapped memory region pointing to this block.
	pub fn as_region(&self) -> MemoryRegion {
		MemoryRegion::new(self.address, self.size)
	}
}

impl Drop for MemoryBlock {
	fn drop(&mut self) {
		extern "system" {
			fn VirtualFree(address: usize, size: usize, free_type: u32) -> u32;
		}

		const MEM_RELEASE: u32 = 0x8000;

		unsafe {
			// Note: if this fails, there is nothing we can do - memory will be leaked.
			// It shouldn't happen, however, since we know the memory address was also allocated by us.
			VirtualFree(self.address, 0, MEM_RELEASE);
		}
	}
}
