use dotenv::dotenv;
use reqwest;
use std;

pub enum Client {
    LocalClient(LocalClient),
    ObsidianClient(ObsidianClient),
}

impl Client {
    pub fn new() -> Client {
        dotenv().ok();
        let client_type = std::env::var("CLIENT_TYPE").expect("CLIENT_TYPE env var not set");
        match client_type.as_str() {
            "local" => Client::LocalClient(LocalClient::new()),
            "obsidian" => Client::ObsidianClient(ObsidianClient::new()),
            _ => panic!("Unsupported client type: {}", client_type),
        }
    }

    pub async fn post(&self, content: String, doc_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Client::LocalClient(client) => {
                client.post_locally(content, doc_name)?;
            }
            Client::ObsidianClient(client) => {
                client.post_to_obsidian(content, doc_name).await?;
            }

        }
        Ok(())
    }
}

pub struct LocalClient {
    out_path: String,
}

impl LocalClient {
    pub fn new() -> LocalClient {
        dotenv().ok();
        LocalClient {
            out_path: std::env::var("OUT_PATH").expect("OUT_PATH env var not set"),
        }
    }

    pub fn post_locally(&self, content: String, doc_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !std::fs::exists(&self.out_path)? {
            std::fs::create_dir_all(&self.out_path)?;
        }

        std::fs::write(format!("{}/{}.txt", &self.out_path, doc_name), content)?;
        Ok(())
    }
}

pub struct ObsidianClient {
    obsidian_url: String,
    auth_token: String,
    vault_name: String,
    client: reqwest::Client,
}

impl ObsidianClient {
    pub fn new() -> ObsidianClient {
        ObsidianClient {
            obsidian_url: std::env::var("OBSIDIAN_URL").expect("OBSIDIAN_URL env var not set"),
            auth_token: std::env::var("OBSIDIAN_API_KEY").expect("OBSIDIAN_API_KEY env var not set"),
            vault_name: std::env::var("OBSIDIAN_VAULT_NAME").expect("OBSIDIAN_VAULT_NAME env var not set"),
            client: reqwest::Client::builder()
                .build()
                .unwrap(),
        }
    }

    pub async fn post_to_obsidian(&self, content: String, doc_name: &str) -> Result<(), reqwest::Error> {
        Self::check_api_status(&self).await?;
        self.client
            .post(format!(
                "{}/{}/{}.md",
                self.obsidian_url, self.vault_name, doc_name
            ))
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .header("Content-Type", "text/plain")
            .body(content)
            .send()
            .await?;
        Ok(())
    }
    async fn check_api_status(&self) -> Result<reqwest::Response, reqwest::Error> {
        dotenv().ok();
        let res = self
            .client
            .get(format!("{}/active", self.obsidian_url))
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;
        Ok(res)
    }
}
