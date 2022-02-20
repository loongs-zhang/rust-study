use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox(t)
    }
}

//实现DerefMut可重载可变引用的*运算
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = &x;
    println!("{}", (x == *y));
    let y = MyBox::new(x);
    //实现Deref，才支持*运算符来解引用
    println!("{}", (x == *y));
    //上下2行，二者是等价的
    println!("{}", (x == *(y.deref())));

    let test = MyBox::new(String::from("test"));
    //这里可以调用，因为发生了解引用的自动转换
    //&MyBox<String> -> &String -> &str
    hello(&test);
}