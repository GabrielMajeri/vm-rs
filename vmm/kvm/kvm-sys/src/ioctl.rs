use macros::*;

kvm_ioctl_none_arg!(get_api_version with 0x00);
kvm_ioctl_none_arg!(check_extension with 0x03);
kvm_ioctl_none_arg!(create_vm with 0x01);

kvm_ioctl_none!(get_vcpu_mmap_size with 0x04);

kvm_ioctl_rw!(get_emulated_cpuid with 0x09; ::CpuidHeader);

kvm_ioctl_none!(create_irq_chip with 0x60);

kvm_ioctl_none_arg!(create_vcpu with 0x41);

kvm_ioctl_r!(get_fpu with 0x8C; ::FpuState);
