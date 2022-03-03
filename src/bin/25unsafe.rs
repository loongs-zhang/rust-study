fn main() {
    let mut num = 5;
    //不可变的原始指针
    let r1 = &num as *const i32;
    //可变的原始指针
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    //pointer指针可能有数据，也可能没数据
    // let pointer = 0x012345usize as *const i32;
    // unsafe {
    //     println!("{}", *pointer);
    // }
}