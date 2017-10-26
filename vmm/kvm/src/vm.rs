use accel;
use global::Global;

pub struct VirtualMachine<'a> {
    global: &'a Global,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(global: &'a Global) -> Self {
        let vm = VirtualMachine { global };

        vm
    }
}

impl<'a> accel::VirtualMachine<'a> for VirtualMachine<'a> {}
