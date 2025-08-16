use dotenv::dotenv;
use reqwest;
use std;
use log::info;

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

    pub async fn post_to_obsidian(&self, content: String, doc_name: &str) -> () {
        let api_status = Self::check_api_status(&self).await;
        match api_status {
            Ok(api_status) => {
                info!("API Status: {}", api_status.status());
            }
            Err(api_error) => {
                panic!("Obsidian API not working due to: {}", api_error);
            }
        }

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
    async fn check_api_status(&self) -> Result<reqwest::Response, reqwest::Error> {
        dotenv().ok();
        let res = self.client
            .get(
                format!("{}/active", self.obsidian_url)
            )
            .header(
                "Authorization",
                format!("Bearer {}", self.auth_token),
            )
            .send()
            .await?;
        Ok(res)
    }
}
