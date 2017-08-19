#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HaxVersion {
	/// V4: supports unmapping and MMIO moves.
	V4 = 4
}

pub const HAX_CURRENT_VERSION: HaxVersion = HaxVersion::V4;
pub const HAX_MINIMUM_VERSION: HaxVersion = HaxVersion::V4;
