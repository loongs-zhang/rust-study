use context::stack::{ProtectedFixedSizeStack};
use context::{Context, Transfer};

// This method will always `resume()` immediately back to the
// previous `Context` with a `data` value incremented by one starting at 0.
// You could thus describe this method as a "natural number generator".
pub extern "C" fn context_function(mut t: Transfer) -> ! {
    loop {
        let i = t.data;
        print!("Resuming {} => ", i);
        t = unsafe { t.context.resume(i) };
    }
}

//可以尝试用libc的pthread来写协程上下文加载
//或者就是直接走汇编：https://github.com/torvalds/linux/blob/v5.2/arch/x86/entry/entry_64.S#L282
fn main() {
    // Allocate some stack.
    let stack = ProtectedFixedSizeStack::new(2048).unwrap();
    // Allocate a Context on the stack.
    let mut t = Transfer::new(unsafe { Context::new(&stack, context_function) }, 0);
    // Yield 10 times to `context_function()`.
    for i in 0..10 {
        // Yield to the "frozen" state of `context_function()`.
        // The first and every other call will return references to the actual `Context` data.
        print!("Yielding {} => ", i);
        t = unsafe { t.context.resume(i) };
        println!("Resumed {}", t.data);
    }
    println!("Finished!");
}
