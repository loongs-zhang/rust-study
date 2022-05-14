//被hook的系统函数
#[no_mangle]
pub extern "C" fn sleep(i: usize) -> usize {
    println!("hooked {}", i);
    i
}