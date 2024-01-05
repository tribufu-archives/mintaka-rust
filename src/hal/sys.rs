// Copyright (c) Tribufu. All Rights Reserved.

use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};

pub(crate) fn get_system_info() -> System {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys
}

/// Returns the uptime of the current system.
pub fn get_system_uptime() -> u64 {
    System::uptime()
}

/// Returns the name of the current system.
pub fn get_system_name() -> Option<String> {
    System::name()
}

/// Returns the OS version of the current system.
pub fn get_system_version() -> Option<String> {
    System::os_version()
}

/// Returns the OS edition of the current system.
pub fn get_system_edition() -> Option<String> {
    System::long_os_version()
}

/// Returns the kernel version of the current system.
pub fn get_kernel_version() -> Option<String> {
    System::kernel_version()
}

/// Returns the host name of the current system.
pub fn get_host_name() -> Option<String> {
    System::host_name()
}

/// Returns the average load of the current system.
pub fn get_cpu_usage() -> f64 {
    let mut sys = get_system_info();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();
    let cpus = sys.cpus();
    cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() as f64 / cpus.len() as f64
}
