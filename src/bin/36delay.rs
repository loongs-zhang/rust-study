use std::thread;
use std::time::Duration;
use scheduled_thread_pool::ScheduledThreadPool;

fn main() {
    let pool = ScheduledThreadPool::new(1);
    pool.execute_after(Duration::from_millis(1000), || {
        println!("delayed");
    });
    thread::sleep(Duration::from_millis(1200));
}