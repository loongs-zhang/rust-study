use crate::register::{align_down, mut_offset};
use crate::register::asm::{bootstrap_green_task, prefetch};
use crate::reg_context::InitFn;
use crate::stack::Stack;

#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    gpr: [usize; 8],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 8] }
    }

    #[inline]
    pub fn prefetch(&self) {
        unsafe {
            prefetch(self as *const _ as *const usize);
            prefetch(self.gpr[1] as *const usize);
        }
    }
}

pub fn initialize_call_frame(
    regs: &mut Registers,
    fptr: InitFn,
    arg: usize,
    arg2: *mut usize,
    stack: &Stack,
) {
    // Redefinitions from rt/arch/x86_64/regs.h
    const RUSTRT_RSP: usize = 1;
    const RUSTRT_RBP: usize = 2;
    const RUSTRT_R12: usize = 4;
    const RUSTRT_R13: usize = 5;
    const RUSTRT_R14: usize = 6;

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[RUSTRT_R12] = arg;
    regs.gpr[RUSTRT_R13] = arg2 as usize;
    regs.gpr[RUSTRT_R14] = fptr as usize;

    // Last base pointer on the stack should be 0
    regs.gpr[RUSTRT_RBP] = 0;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[RUSTRT_RSP] = mut_offset(sp, -2) as usize;

    unsafe {
        // leave enough space for RET
        *mut_offset(sp, -2) = bootstrap_green_task as usize;
        *mut_offset(sp, -1) = 0;
    }
}
