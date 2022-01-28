use std::collections::HashMap;

pub async fn request(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{}\n{:#?}", id, resp);
    Ok(())
}