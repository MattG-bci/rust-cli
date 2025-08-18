use crate::api::client::ObsidianClient;
use dotenv::dotenv;

pub async fn post_llm_response(name: &str, content: String) -> () {
    dotenv().ok();
    let client = ObsidianClient::new();
    client.post_to_obsidian(content, name).await;
}
