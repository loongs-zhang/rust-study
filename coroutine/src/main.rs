use std::collections::HashMap;
use std::rc::Rc;
use context::stack::StackError;
use coroutine::scheduler::Scheduler;
use coroutine::coroutine::Coroutine;

fn main() -> Result<(), StackError> {
    let mut scheduler = Scheduler::new();
    let mut context: HashMap<&str, Rc<Coroutine>> = HashMap::new();
    let func = |param: i32| -> i32 {
        let c1 = context.get("c1").unwrap();
        let c2 = context.get("c2").unwrap();
        println!("{:?}", c1);
        println!("{:?}", c2);

        println!("{}", param);
        return param * 2;
    };
    //fixme
    let c1 = Coroutine::new(func, &mut scheduler)?;
    let c2 = Coroutine::new(func, &mut scheduler)?;
    context.insert("c1", c1);
    context.insert("c2", c2);
    Ok(())
}
