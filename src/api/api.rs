use crate::api::client::Client;
use dotenv::dotenv;
use std::error::Error;

pub async fn post_llm_response(name: &str, content: String) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let client = Client::new();
    client.post(content, name).await
}
