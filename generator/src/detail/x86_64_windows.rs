use crate::detail::{align_down, bootstrap_green_task, mut_offset};
use crate::reg_context::InitFn;
use crate::stack::Stack;

#[inline]
#[allow(dead_code)]
pub fn prefetch(data: *const usize) {
    unsafe { crate::detail::prefetch(data) }
}

// #[cfg_attr(nightly, repr(simd))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Xmm(u32, u32, u32, u32);

impl Xmm {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Xmm(a, b, c, d)
    }
}

// windows need to restore xmm6~xmm15, for most cases only use two xmm registers
#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    gpr: [usize; 16],
    // keep enough for place holder
    _xmm: [Xmm; 10],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            gpr: [0; 16],
            _xmm: [Xmm::new(0, 0, 0, 0); 10],
        }
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
    const RUSTRT_STACK_BASE: usize = 11;
    const RUSTRT_STACK_LIMIT: usize = 12;
    const RUSTRT_STACK_DEALLOC: usize = 13;

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[RUSTRT_R12] = arg;
    regs.gpr[RUSTRT_R13] = arg2 as usize;
    regs.gpr[RUSTRT_R14] = fptr as usize;

    // Last base pointer on the stack should be 0
    regs.gpr[RUSTRT_RBP] = 0;

    regs.gpr[RUSTRT_STACK_BASE] = stack.end() as usize;
    regs.gpr[RUSTRT_STACK_LIMIT] = stack.begin() as usize;
    regs.gpr[RUSTRT_STACK_DEALLOC] = 0; //mut_offset(sp, -8192) as usize;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[RUSTRT_RSP] = mut_offset(sp, -2) as usize;

    unsafe {
        // leave enough space for RET
        *mut_offset(sp, -2) = bootstrap_green_task as usize;
        *mut_offset(sp, -1) = 0;
    }
}
