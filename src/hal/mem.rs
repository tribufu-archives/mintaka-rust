// Copyright (c) Tribufu. All Rights Reserved.

use crate::sys;

/// Returns the total memory of the current system.
pub fn get_total_memory() -> u64 {
    sys::get_system_info().total_memory()
}

/// Returns the free memory of the current system.
pub fn get_free_memory() -> u64 {
    sys::get_system_info().free_memory()
}

/// Returns the used memory of the current system.
pub fn get_used_memory() -> u64 {
    sys::get_system_info().used_memory()
}

/// Returns the total swap of the current system.
pub fn get_total_swap() -> u64 {
    sys::get_system_info().total_swap()
}

/// Returns the free swap of the current system.
pub fn get_free_swap() -> u64 {
    sys::get_system_info().free_swap()
}

/// Returns the used swap of the current system.
pub fn get_used_swap() -> u64 {
    sys::get_system_info().used_swap()
}
