//! Supports creating virtual machines using KVM (Kernel-based Virtual Machines).

#![cfg(target_os = "linux")]

#![cfg_attr(feature="clippy", deny(warnings, missing_docs, clippy))]
