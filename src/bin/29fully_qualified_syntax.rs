trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Dog")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Animal")
    }
}

fn main() {
    println!("{}", Dog::baby_name());
    //使用完全限定语法来调用Dog为Animal trait实现的baby_name函数
    println!("{}", <Dog as Animal>::baby_name());
}