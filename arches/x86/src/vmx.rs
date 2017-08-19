/// These are the VMX basic exit reasons.
///
/// You can find them in the appendix from the Intel Architecture Manual, Vol. 3.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ExitReason {
	/// Guest software caused an exception which requires the VM to exit.
	Exception = 0,
	/// An external interrupt arrived.
	ExternalInterrupt = 1,

	/// The logical processor encountered an exception while attempting to call the double-fault handler and
	/// that exception did not itself cause a VM exit due to the exception bitmap.
	TripleFault = 2,

	/// An INIT signal arrived.
	INITSignal = 3,

	/// An attempt to access memory with a guest-physical address was disallowed by the configuration of the EPT paging structures.
	EptViolation = 48,
}
