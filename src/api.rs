use std::fmt::format;
use reqwest;
use reqwest::Response;
use dotenv::dotenv;

pub async fn check_api_status() -> Response {
    dotenv().ok();
    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let res = client
        .get("https://127.0.0.1:27124/active")
        .header("Authorization", format!("Bearer {}", std::env::var("OBSIDIAN_API_KEY").unwrap()))
        .send()
        .await
        .unwrap();
    res
}




