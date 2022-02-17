use std::fmt::Display;

pub trait TestInterface {
    fn hello<'a>(&self, param: &'a str) -> &'a str;

    //允许默认实现
    fn default(&self) {
        println!("this is a default implement");
        println!("{}", self.hello("ss"));
    }
}

struct TestInterfaceImpl {}

impl TestInterface for TestInterfaceImpl {
    fn hello<'a>(&self, param: &'a str) -> &'a str {
        param
    }

    fn default(&self) {
        //无法从这里调用TestInterface的默认实现
    }
}

//如果要求返回类型要实现多个trait，只能使用这种办法
fn returnTrait<T>() -> T
    where T: TestInterface + Display
{
    todo!()
}

fn param1(param1: impl TestInterface + Display) {}

//对param1的改进写法
fn param2<T: TestInterface + Display>(param1: T, param2: T) {}

fn param3<T: TestInterface + Display, U: TestInterface + Clone>(param1: T, param2: U) -> i32 {
    return 1;
}

//建议写法
fn param4<T, U>(param1: T, param2: U) -> i32
    where T: TestInterface + Display,
          U: TestInterface + Clone
{
    return 1;
}

fn main() {
    let obj = TestInterfaceImpl {};
    println!("{}", obj.hello("test"));
    obj.default();
}