fn pointer_move() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1指针"移动"到了s2上
    // 注意：函数在返回值的过程中也会发生所有权的转移
    //println!("{}, world!", s1);
    //                       ^^ value borrowed here after move
}

fn reference(s: &mut str) -> usize {
    //这里s传入的是引用，不会引起所有权的转移
    s.len()
}

fn borrow_twice() {
    let mut s = String::from("hello");
    //现在一次可以声明多个可变引用了
    let r1 = &mut s;
    let r2 = &mut s;
}

fn mut_and_unmut() {
    let mut s = String::from("hello");
    //现在可以同时声明可变引用和不可变引用了
    let r1 = &s;
    let r2 = &mut s;
}

fn main() {
    pointer_move();
    println!("{}", reference(&mut String::from("haha")));
    borrow_twice();
    mut_and_unmut();
}