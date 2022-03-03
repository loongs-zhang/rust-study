use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let c1 = Arc::clone(&mutex);
    let c2 = Arc::clone(&mutex);
    let h1 = thread::spawn(move || {
        for _ in 0..100 {
            let mut num = c1.lock().unwrap();
            *num = *num + 1;
        }
    });
    let h2 = thread::spawn(move || {
        for _ in 0..100 {
            let mut num = c2.lock().unwrap();
            *num = *num + 1;
        }
    });
    h1.join().unwrap();
    h2.join().unwrap();
    println!("result:{:?}", mutex.lock().unwrap());
}