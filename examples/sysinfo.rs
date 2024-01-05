// Copyright (c) Tribufu. All Rights Reserved.

use mintaka_hal::*;

fn main() {
    println!("Physical cores: {}", cpu::get_physical_cores());
    println!("Logical cores: {}", cpu::get_logical_cores());
    println!("Processor model: {}", cpu::get_processor_model());
    println!("Processor vendor: {}", cpu::get_processor_vendor());
    println!(
        "Total memory: {} GB",
        (mem::get_total_memory() as f64 / 1073741824.0 * 10.0).round() / 10.0
    );
    println!(
        "Free memory: {} GB",
        (mem::get_free_memory() as f64 / 1073741824.0 * 10.0).round() / 10.0
    );
    println!(
        "Used memory: {} GB",
        (mem::get_used_memory() as f64 / 1073741824.0 * 10.0).round() / 10.0
    );
    println!(
        "Total swap: {} GB",
        (mem::get_total_swap() as f64 / 1073741824.0 * 10.0).round() / 10.0
    );
    println!(
        "Free swap: {} GB",
        (mem::get_free_swap() as f64 / 1073741824.0 * 10.0).round() / 10.0
    );
    println!(
        "Used swap: {} GB",
        (mem::get_used_swap() as f64 / 1073741824.0 * 10.0).round() / 10.0
    );
    println!("System uptime: {} hours", sys::get_system_uptime() / 3600);
    println!("System name: {}", sys::get_system_name().unwrap());
    println!("System version: {}", sys::get_system_version().unwrap());
    println!("System edition: {}", sys::get_system_edition().unwrap());
    println!("Kernel version: {}", sys::get_kernel_version().unwrap());
    println!("Host name: {}", sys::get_host_name().unwrap());
}
