use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use context::stack::{ProtectedFixedSizeStack, StackError};
use context::{Context, Transfer};
use coroutine::scheduler::Scheduler;
use coroutine::coroutine::Coroutine;

fn simple() {
    // This method will always `resume()` immediately back to the
    // previous `Context` with a `data` value incremented by one starting at 0.
    // You could thus describe this method as a "natural number generator".
    extern "C" fn context_function(mut t: Transfer) -> ! {
        for i in 0usize.. {
            print!("Yielding {} => ", i);
            t = unsafe { t.context.resume(i) };
        }

        unreachable!();
    }

    // Allocate some stack.
    let stack = ProtectedFixedSizeStack::default();

    // Allocate a Context on the stack.
    let mut t = Transfer::new(unsafe { Context::new(&stack, context_function) }, 0);

    // Yield 10 times to `context_function()`.
    for _ in 0..10 {
        // Yield to the "frozen" state of `context_function()`.
        // The `data` value is not used in this example and is left at 0.
        // The first and every other call will return references to the actual `Context` data.
        print!("Resuming => ");
        t = unsafe { t.context.resume(0) };

        println!("Got {}", t.data);
    }

    println!("Finished!");
}

fn func(context: HashMap<&str, String>) -> i32 {
    let c1 = context.get("c1").unwrap();
    let c2 = context.get("c2").unwrap();
    println!("{:?}", c1);
    println!("{:?}", c2);
    return 0;
}

fn main() -> Result<(), String> {
    // let scheduler = Scheduler::new();
    // let mut context: HashMap<&str, String> = HashMap::new();
    // context.insert("c1", String::from("c1"));
    // context.insert("c2", String::from("c2"));
    // let c1 = Coroutine::new(func, Rc::new(RefCell::new(&scheduler)))?;
    // let c2 = Coroutine::new(func, Rc::new(RefCell::new(&scheduler)))?;
    // scheduler.start();
    simple();
    Ok(())
}
