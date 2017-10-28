//! Architecture-specific types.

use x86;

/// Structure representing the virtual CPU's state,
/// including general and system registers, floating-point units
/// or other data.
pub type CpuState = x86::state::State;

/// An architecture-specific number representing the reason why
/// the virtual CPU stopped execution.
pub type ExitReason = x86::vmx::ExitReason;
