use std::fmt;

trait OutlinePrint: fmt::Debug {
    fn outline_print(&self) {
        println!("* {:?} *", self);
    }
}

//使用注解隐式实现
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Point如果要实现OutlinePrint，则必须实现Display
// 这里Display就是"super trait"
impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.outline_print();
}