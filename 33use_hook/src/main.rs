fn main() {
    unsafe {
        println!("{}", libc::abs(-1));
    }
}