use crate::api::client::APIClient;
use dotenv::dotenv;
use reqwest;
use reqwest::Response;


pub async fn post_llm_response(name: &str, content: String) -> () {
    dotenv().ok();
    let client = APIClient::new();
    client.post_to_obsidian(content, name).await;
}
