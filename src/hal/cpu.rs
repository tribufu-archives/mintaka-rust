// Copyright (c) Tribufu. All Rights Reserved.

use crate::sys;

/// Returns the number of physical cores of the current CPU.
pub fn get_physical_cores() -> usize {
    num_cpus::get_physical()
}

/// Returns the number of logical cores of the current CPU.
pub fn get_logical_cores() -> usize {
    num_cpus::get()
}

/// Returns the model of the current CPU.
pub fn get_processor_model() -> String {
    let mut sys = sys::get_system_info();
    sys.refresh_cpu();
    sys.cpus()[0].brand().trim().to_string()
}

/// Returns the vendor of the current CPU.
pub fn get_processor_vendor() -> String {
    let mut sys = sys::get_system_info();
    sys.refresh_cpu();
    sys.cpus()[0].vendor_id().trim().to_string()
}
