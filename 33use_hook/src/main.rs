fn main() {
    unsafe {
        println!("{}", libc::sleep(1));
    }
}