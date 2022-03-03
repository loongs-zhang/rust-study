extern "C" {
    fn abs(i: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", abs(-1));
        //调用c语言的函数
        //c语言调用rust见本项目的module2
        println!("{}", libc::abs(-2));
    }
}