use context::stack::{ProtectedFixedSizeStack};
use context::{Context, Transfer};

// This method will always `resume()` immediately back to the
// previous `Context` with a `data` value incremented by one starting at 0.
// You could thus describe this method as a "natural number generator".
extern "C" fn context_function(mut t: Transfer) -> ! {
    loop {
        let i = t.data;
        print!("Resuming {} => ", i);
        t = unsafe { t.context.resume(i) };
    }
}

fn main() {
    // Allocate some stack.
    let stack = ProtectedFixedSizeStack::default();
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
