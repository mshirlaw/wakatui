use base64::{Engine as _, engine::general_purpose::STANDARD};
use color_eyre::eyre::{Result, eyre};
use reqwest::blocking::Client;

use super::storage::ApiKeyStorage;

#[derive(Debug)]
pub struct AuthController {
    storage: ApiKeyStorage,
    client: Client,
}

impl AuthController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            storage: ApiKeyStorage::new()?,
            client: Client::new(),
        })
    }

    pub fn set_api_key(&mut self, api_key: String) -> Result<()> {
        self.validate_api_key(&api_key)?;
        self.storage.save_api_key(&api_key)?;

        Ok(())
    }

    pub fn validate_api_key(&self, api_key: &str) -> Result<()> {
        let auth_header = format!("Basic {}", STANDARD.encode(api_key.as_bytes()));

        let response = self
            .client
            .get("https://api.wakatime.com/api/v1/users/current")
            .header("Authorization", auth_header)
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(eyre!("Invalid API key. Status: {}", response.status()))
        }
    }

    pub fn get_api_key(&self) -> Result<Option<String>> {
        self.storage.load_api_key()
    }
}
