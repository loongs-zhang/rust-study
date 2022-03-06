use crate::register::InitFunction;
use crate::register::system::{align_down, bootstrap_green_task, prefetch};
use crate::stack::Stack;

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Registers {
    // We save the 13 callee-saved registers:
    //  x19--x28, fp (x29), lr (x30), sp
    // and the 8 callee-saved floating point registers:
    //  v8--v15
    gpr: [usize; 32],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 32] }
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
    fptr: InitFunction,
    arg: usize,
    arg2: *mut usize,
    stack: &Stack,
) {
    // Callee-saved registers start at x19
    const X19: usize = 19 - 19;
    const X20: usize = 20 - 19;
    const X21: usize = 21 - 19;

    const FP: usize = 29 - 19;
    const LR: usize = 30 - 19;
    const SP: usize = 31 - 19;

    let sp = align_down(stack.top());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[X19] = arg;
    regs.gpr[X20] = arg2 as usize;
    regs.gpr[X21] = fptr as usize;

    // Aarch64 current stack frame pointer
    regs.gpr[FP] = sp as usize;

    regs.gpr[LR] = bootstrap_green_task as usize;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[SP] = sp as usize;
}
