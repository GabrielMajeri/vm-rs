use macros::*;

kvm_ioctl_none!(get_api_version with 0x00);
kvm_ioctl_none!(check_extension with 0x03);

kvm_ioctl!(get_vcpu_mmap_size with 0x04);

kvm_ioctl_rw!(get_emulated_cpuid with 0x09; ::CpuidHeader);
