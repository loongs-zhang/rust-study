extern "C" {
    fn abs(i: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", abs(-1));
        //调用c语言的函数
        println!("{}", libc::abs(-2));
    }
}