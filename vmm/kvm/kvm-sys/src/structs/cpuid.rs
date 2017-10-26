use std::fmt;

/// Header of CPUID entries array.
///
/// You must allocate a contiguous array of `CpuidEntry`,
/// with this structure as a header.
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct CpuidHeader {
    /// Number of entries in the array.
    pub len: u32,
    _padding: u32,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct CpuidEntry {
    /// The value of EAX used to obtain this entry.
    pub function: u32,
    /// The value of ECX used to obtain this entry (where relevant).
    pub index: u32,
    pub flags: CpuidFlag,
    /// The registers EAX, EBX, ECX, and EDX returned for this function.
    pub r: [u32; 4],
    _padding: [u32; 3],
}

impl fmt::Debug for CpuidEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CpuidEntry {{ ")?;
        write!(f, "fn: {}, ", self.function)?;

        // Print index, if necessary.
        let mut flags = self.flags;
        let has_index = CpuidFlag::SIGNIFICANT_INDEX;

        if self.flags.contains(has_index) {
            write!(f, "index: {}, ", self.index)?;

            // Remove the significant index flag.
            flags.remove(has_index);
        }

        write!(f, "flags: {:?}, ", flags)?;
        write!(f, "regs: {:?} ", self.r)?;
        write!(f, "}}")
    }
}

bitflags! {
    /// Flags for a CPUID entry.
    #[derive(Default)]
    pub struct CpuidFlag: u32 {
        /// The `index` field is valid.
        const SIGNIFICANT_INDEX = 1;
        /// CPUID returns different values for this function on successive invocations.
        ///
        /// The array will contain multiple entries with this function,
        /// all of them will have this flag set.
        const STATEFUL = 2;
        /// For entries which are `STATEFUL`, this marks the next entry
        /// the CPU will read.
        const STATEFUL_READ_NEXT = 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpuid_entry_debug_simple() {
        let mut entry = CpuidEntry::default();

        entry.function = 1;
        entry.r = [12, 0, 4, 5];

        let s = format!("{:?}", entry);
        let exp = "CpuidEntry { fn: 1, flags: (empty), regs: [12, 0, 4, 5] }";

        assert_eq!(s, exp);

        entry.function = 10;
        entry.flags |= CpuidFlag::SIGNIFICANT_INDEX;
        entry.index = 12;

        let s = format!("{:?}", entry);
        let exp = "CpuidEntry { fn: 10, index: 12, flags: (empty), regs: [12, 0, 4, 5] }";

        assert_eq!(s, exp);
    }
}
