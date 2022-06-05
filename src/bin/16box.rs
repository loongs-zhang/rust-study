use std::alloc::{alloc, Layout};

/**
Box<T>适用场景：
1.在编译时，无法确定类型的大小，但使用这个类型的值时，上下文又需要知道它的大小；
2.需要传递大量数据的所有权，但又不希望产生大量数据的复制行为；
3.当你希望拥有一个实现了指定trait的类型值，但又不关心具体的类型。
 */

enum List {
    //这里Box<List>的大小是指针的大小
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    //Box<T>会把数据存储在heap上，stack上只留下指针
    let boxed = Box::new(5);
    println!("{}", boxed);
    let test = || {
        println!("test");
    };
    let text = Box::new(test);
    let ptr = Box::into_raw(text);
    unsafe {
        (*ptr)();
    }
}//在代码运行到这里时，boxed对应的stack里指针和heap里的数据都会被清理