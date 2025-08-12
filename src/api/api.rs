use crate::api::client::APIClient;
use dotenv::dotenv;
use reqwest;
use reqwest::Response;

pub async fn check_api_status() -> Response {
    dotenv().ok();
    let client = reqwest::Client::builder().build().unwrap();
    let res = client
        .get("https://127.0.0.1:27124/active")
        .header(
            "Authorization",
            format!("Bearer {}", std::env::var("OBSIDIAN_API_KEY").unwrap()),
        )
        .send()
        .await
        .unwrap();
    res
}

pub async fn post_llm_response(name: &str, content: String) -> () {
    dotenv().ok();
    let client = APIClient::new();
    client.post(content, name).await;
}
