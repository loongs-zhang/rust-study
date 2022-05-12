//被hook的系统函数
#[no_mangle]
pub extern "C" fn abs(i: i32) -> i32 {
    println!("hooked {}", i);
    i
}