use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        thread::sleep(Duration::from_millis(20));
        let mut started = lock.lock().unwrap();
        *started = true;
        println!("started");
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    loop {
        println!("loop");
        let result = cvar.wait_timeout_ms(started, 10).unwrap();
        // 10 milliseconds have passed, or maybe the value changed!
        started = result.0;
        if *started == true {
            // We received the notification and the value has been updated, we can leave.
            println!("break");
            break;
        }
    }
    println!("finished");
}