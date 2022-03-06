// Register contexts used in various architectures
//
// These structures all represent a context of one task throughout its
// execution. Each struct is a representation of the architecture's register
// set. When swapping between tasks, these register sets are used to save off
// the current registers into one struct, and load them all from another.
//
// Note that this is only used for context switching, which means that some of
// the registers may go unused. For example, for architectures with
// callee/caller saved registers, the context will only reflect the callee-saved
// registers. This is because the caller saved registers are already stored
// elsewhere on the stack (if it was necessary anyway).
//
// Additionally, there may be fields on various architectures which are unused
// entirely because they only reflect what is theoretically possible for a
// "complete register set" to show, but user-space cannot alter these registers.
// An example of this would be the segment selectors for x86.
//
// These structures/functions are roughly in-sync with the source files inside
// of src/rt/arch/$arch. The only currently used function from those folders is
// the `rust_swap_registers` function, but that's only because for now segmented
// stacks are disabled.

use std::ffi::c_void;
use crate::register::system::asm::Registers;

#[cfg(all(unix, target_arch = "x86_64"))]
#[path = "unix_x86_64.rs"]
pub mod asm;

#[cfg(all(windows, target_arch = "x86_64"))]
#[path = "windows_x86_64.rs"]
pub mod asm;

#[cfg(all(unix, target_arch = "aarch64"))]
#[path = "unix_aarch64.rs"]
pub mod asm;

#[link(name = "asm", kind = "static")]
extern "C" {
    pub fn bootstrap_green_task();
    pub fn prefetch(data: *const usize);
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

#[inline]
fn align_down(sp: *mut c_void) -> *mut usize {
    let sp = (sp as usize) & !(16 - 1);
    sp as *mut usize
}

// ptr::mut_offset is positive isizes only
#[inline]
#[allow(unused)]
fn mut_offset<T>(ptr: *mut T, count: isize) -> *mut T {
    // use std::mem::size_of;
    // (ptr as isize + count * (size_of::<T>() as isize)) as *mut T
    unsafe { ptr.offset(count) }
}