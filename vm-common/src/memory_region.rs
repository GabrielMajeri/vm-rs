use { HostAddress, GuestAddress };

/// Represents a region of host memory that is mapped in a virtual machine.
pub struct MemoryRegion {
	host_addr: HostAddress,
	guest_addr: GuestAddress,
	size: usize
}

impl MemoryRegion {
	/// Creates a new, unmapped memory region.
	pub fn new(host_addr: HostAddress, size: usize) -> Self {
		MemoryRegion {
			host_addr,
			guest_addr: 0,
			size
		}
	}

	/// Changes the region's mapping.
	pub fn map(&mut self, guest_addr: GuestAddress) {
		self.guest_addr = guest_addr;
	}

	/// The address of the host memory.
	pub fn host_address(&self) -> HostAddress {
		self.host_addr
	}

	/// The address of the mapped guest memory.
	pub fn guest_address(&self) -> GuestAddress {
		self.guest_addr
	}

	/// The size, in bytes, of this memory region.
	pub fn size(&self) -> usize {
		self.size
	}
}
