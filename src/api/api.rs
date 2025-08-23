use crate::api::client::Client;
use dotenv::dotenv;

pub async fn post_llm_response(name: &str, content: String) -> () {
    dotenv().ok();
    let client = Client::new();
    client.post(content, name).await.ok();
}
