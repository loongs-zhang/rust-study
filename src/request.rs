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
    request("16".to_string()),
    request("17".to_string()),
    request("18".to_string()),
    request("19".to_string()),
    request("20".to_string()),
    request("21".to_string()),
    request("22".to_string()),
    request("23".to_string()),
    request("24".to_string()),
    request("25".to_string()),
    request("26".to_string()),
    request("27".to_string()),
    request("28".to_string()),
    request("29".to_string()),
    request("30".to_string()),
    request("31".to_string()),
    request("32".to_string()),);
    // cost:4000+ms, therefore, it is concluded that
    // all stackless coroutines are fake coroutines,
    // and rust is no exception, and the stack coroutine
    // represented by go is the real coroutine
    println!("cost:{:?}ms", current_time_millis() - start);
}

pub async fn request(id: String) -> Result<(), Box<dyn std::error::Error>> {
    // see https://github.com/dragon-zhang/kotlin-study/blob/master/provider/src/main/kotlin/com/example/demo/TestController.kt
    let resp = reqwest::get("http://127.0.0.1:8081/rust")
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