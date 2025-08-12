use reqwest;

pub struct APIClient {
    obsidian_url: String,
    auth_token: String,
    vault_name: String,
    client: reqwest::Client,
}

impl APIClient {
    pub fn new() -> APIClient {
        APIClient {
            obsidian_url: std::env::var("OBSIDIAN_URL").unwrap(),
            auth_token: std::env::var("OBSIDIAN_API_KEY").unwrap(),
            vault_name: std::env::var("OBSIDIAN_VAULT_NAME").unwrap(),
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn get(&self, path: &str) -> () {}

    pub async fn post(&self, content: String, doc_name: &str) -> () {
        self.client
            .post(format!(
                "{}/{}/{}.md",
                self.obsidian_url, self.vault_name, doc_name
            ))
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .header("Content-Type", "text/plain")
            .body(content)
            .send()
            .await
            .unwrap();
    }
}
