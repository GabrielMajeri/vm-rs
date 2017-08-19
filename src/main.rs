//! Virtual machine framework in Rust.

#![cfg(target_os = "windows")]

extern crate env_logger;

extern crate hax;
extern crate vm_common as common;
extern crate x86;

fn main() {
	env_logger::init().expect("Failed to initialize logger");

	let connection = hax::Connection::new().unwrap();

	let vm = hax::VirtualMachine::new(&connection).unwrap();

	let vcpu = hax::vcpu::VirtualCPU::new(&vm, 0).unwrap();

	let mut cpu_state = unsafe { std::mem::uninitialized() };
	vcpu.get_cpu_state(&mut cpu_state).unwrap();
	println!("{:#?}", cpu_state);
}
