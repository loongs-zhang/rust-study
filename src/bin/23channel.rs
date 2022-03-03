use std::sync::mpsc;
use std::thread;

fn main() {
    //mpsc multiple producer, single consumer
    let (sender, receiver) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        sender.send("hello").unwrap();
    });
    thread::spawn(move || {
        s1.send("world").unwrap();
    });
    //try_recv不会阻塞当前线程，对CPU不友好
    for message in receiver {
        println!("got:{:?}", message);
    }
}