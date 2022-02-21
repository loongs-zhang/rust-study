struct AutoDrop<'a> {
    data: &'a str,
}

impl<'a> Drop for AutoDrop<'a> {
    //drop方法不允许手动调用
    fn drop(&mut self) {
        println!("try to drop data:{}", self.data);
    }
}

fn main() {
    let a = AutoDrop { data: "1" };
    let b = AutoDrop { data: "2" };
    //标准库的drop方法可以手动调用
    //drop(a);
    println!("AutoDrop created");
}//观察结果可以发现，drop的调用顺序与变量的声明顺序相反