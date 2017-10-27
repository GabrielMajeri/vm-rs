//! Architecture-specific types.

use x86;

/// Structure representing the virtual CPU's state,
/// including general and system registers, floating-point units
/// or other data.
pub type CpuState = x86::state::State;
