//#[no_mangle]避免rust编译器修改方法名称
#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    println!("hello from rust");
    x + y
}