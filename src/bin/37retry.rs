use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

async fn action() -> Result<u64, ()> {
    println!("test");
    Err(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let retry_strategy = ExponentialBackoff::from_millis(10)
        .map(jitter) // add jitter to delays
        .take(3);    // limit to 3 retries

    let result = Retry::spawn(retry_strategy, action).await?;

    Ok(())
}