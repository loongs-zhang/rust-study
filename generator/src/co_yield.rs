//! # yield
//!
//! generator yield implementation
//!
use std::any::Any;
use crate::reg_context::RegContext;
use crate::runtime::{is_generator, Context, ContextStack, Error};

/// it's a special return instruction that yield nothing
/// but only terminate the generator safely
#[macro_export]
macro_rules! done {
    () => {{
        return $crate::done();
    }};
}

/// don't use it directly, use done!() macro instead
/// would panic if use in none generator context
#[doc(hidden)]
#[inline]
pub fn done<T>() -> T {
    assert!(is_generator(), "done is only possible in a generator");
    // set the done bit for this special return
    ContextStack::current().top()._ref = 0xf;
    // this return value would not be dropped when _ref is 0xf
    // so it's safe here to reutrn a dummy T
    let ret = std::mem::MaybeUninit::uninit();
    unsafe { ret.assume_init() }
}

/// switch back to parent context
#[inline]
pub fn yield_now() {
    let env = ContextStack::current();
    let cur = env.top();
    raw_yield_now(&env, cur);
}

#[inline]
pub fn raw_yield_now(env: &ContextStack, cur: &mut Context) {
    let parent = env.pop_context(cur as *mut _);
    RegContext::swap(&mut cur.regs, &parent.regs);
}

/// coroutine yield
pub fn co_yield_with<T: Any>(v: T) {
    let env = ContextStack::current();
    let context = env.co_ctx().unwrap();

    // check the context, already checked in co_ctx()
    // if !context.is_generator() {
    //     info!("yield from none coroutine context");
    //     // do nothing, just return
    //     return;
    // }

    // here we just panic to exit the func
    if context._ref != 1 {
        std::panic::panic_any(Error::Cancel);
    }

    context.co_set_ret(v);
    context._ref -= 1;

    let parent = env.pop_context(context);
    let top = unsafe { &mut *context.parent };
    // here we should use the top regs
    RegContext::swap(&mut top.regs, &parent.regs);
}

/// coroutine get passed in yield para
pub fn co_get_yield<A: Any>() -> Option<A> {
    ContextStack::current()
        .co_ctx()
        .and_then(|ctx| ctx.co_get_para())
}

/// set current coroutine para in user space
pub fn co_set_para<A: Any>(para: A) {
    if let Some(ctx) = ContextStack::current().co_ctx() {
        ctx.co_set_para(para)
    }
}
