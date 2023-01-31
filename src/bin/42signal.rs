use std::os::unix::thread::JoinHandleExt;
use std::thread;
use std::time::Duration;

static mut RUN: bool = true;

unsafe fn test() {
    println!("{:?}", backtrace::Backtrace::new());
    println!("change flag");
    RUN = false;
}

fn main() -> thread::Result<()> {
    let pthread = thread::spawn(|| unsafe {
        println!("register signal");
        libc::signal(libc::SIGURG, test as usize);
        println!("start loop");
        while RUN {}
        println!("finished");
    }).as_pthread_t();
    thread::sleep(Duration::from_millis(1000));
    thread::spawn(move || unsafe {
        libc::pthread_kill(pthread, libc::SIGURG);
        println!("pthread signaled");
        thread::sleep(Duration::from_millis(1000));
    }).join()
}