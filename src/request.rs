use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
pub async fn test() {
    let start = current_time_millis();
    tokio::join!(request("1".to_string()),
    request("2".to_string()),
    request("3".to_string()),
    request("4".to_string()),
    request("5".to_string()),
    request("6".to_string()),
    request("7".to_string()),
    request("8".to_string()),
    request("9".to_string()),
    request("10".to_string()),
    request("11".to_string()),
    request("12".to_string()),
    request("13".to_string()),
    request("14".to_string()),
    request("15".to_string()),
    request("16".to_string()),);
    println!("cost:{:?}ms", current_time_millis() - start);
}

pub async fn request(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{}\n{:#?}", id, resp);
    Ok(())
}

pub fn current_time_millis() -> i64 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    return ms;
}