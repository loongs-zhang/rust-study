use crate::register::system::asm::{initialize_call_frame, Registers};
use crate::register::system::swap_registers;
use crate::stack::Stack;

mod system;

// first argument is task handle, second is thunk ptr
pub type InitFunction = fn(usize, *mut usize) -> !;

#[derive(Debug)]
pub struct RegisterContext {
    /// Hold the registers while the task or scheduler is suspended
    registers: Registers,
}

impl RegisterContext {
    pub fn empty() -> RegisterContext {
        RegisterContext {
            registers: Registers::new(),
        }
    }

    #[inline]
    pub fn prefetch(&self) {
        self.registers.prefetch();
    }

    /// Create a new context
    pub fn new(init: InitFunction, arg: usize, start: *mut usize, stack: &Stack) -> RegisterContext {
        let mut ctx = RegisterContext::empty();
        ctx.init_with(init, arg, start, stack);
        ctx
    }

    /// init the generator register
    #[inline]
    pub fn init_with(&mut self, init: InitFunction, arg: usize, start: *mut usize, stack: &Stack) {
        // Save and then immediately load the current context,
        // which we will then modify to call the given function when restoredtack
        initialize_call_frame(&mut self.registers, init, arg, start, stack);
    }

    /// Switch contexts
    ///
    /// Suspend the current execution context and resume another by
    /// saving the registers values of the executing thread to a Context
    /// then loading the registers from a previously saved Context.
    #[inline]
    pub fn swap(out_context: &mut RegisterContext, in_context: &RegisterContext) {
        // debug!("register raw swap");
        unsafe { swap_registers(&mut out_context.registers, &in_context.registers) }
    }

    /// Load the context and switch. This function will never return.
    #[inline]
    pub fn load(to_context: &RegisterContext) {
        let mut cur = Registers::new();
        let regs: &Registers = &to_context.registers;

        unsafe { swap_registers(&mut cur, regs) }
    }
}

#[cfg(test)]
mod test {
    use std::mem::transmute;
    use crate::register::RegisterContext;
    use crate::stack::Stack;

    const MIN_STACK: usize = 1024;

    fn init_fn(arg: usize, f: *mut usize) -> ! {
        let func: fn() = unsafe { transmute(f) };
        func();

        let ctx: &RegisterContext = unsafe { transmute(arg) };
        RegisterContext::load(ctx);

        unreachable!("Should never comeback");
    }

    #[test]
    fn test_swap_context() {
        static mut VAL: bool = false;
        let mut cur = RegisterContext::empty();

        fn callback() {
            unsafe {
                VAL = true;
            }
        }

        let stk = Stack::new(MIN_STACK, true).unwrap();
        let ctx = RegisterContext::new(
            init_fn,
            unsafe { transmute(&cur) },
            unsafe { transmute(callback as usize) },
            &stk,
        );

        RegisterContext::swap(&mut cur, &ctx);
        unsafe {
            assert!(VAL);
        }
    }
}
