use color_eyre::eyre::Result;
use keyring::Entry;
use std::fs;
use std::path::PathBuf;

const SERVICE_NAME: &str = "wakatui";
const ACCOUNT_NAME: &str = "api_key";

#[derive(Debug)]
pub struct ApiKeyStorage;

impl ApiKeyStorage {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn load_api_key(&self) -> Result<Option<String>> {
        if let Ok(api_key) = std::env::var("WAKATIME_API_KEY") {
            if !api_key.is_empty() {
                return Ok(Some(api_key));
            }
        }

        if let Some(api_key) = self.load_from_wakatime_cfg()? {
            return Ok(Some(api_key));
        }

        let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)?;
        match entry.get_password() {
            Ok(api_key) => Ok(Some(api_key)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn load_from_wakatime_cfg(&self) -> Result<Option<String>> {
        let home_dir = match std::env::var("HOME") {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => return Ok(None),
        };

        let config_path = home_dir.join(".wakatime.cfg");

        if !config_path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(config_path)?;

        for line in contents.lines() {
            let line = line.trim();
            if line.starts_with("api_key") {
                if let Some(key_value) = line.split('=').nth(1) {
                    let api_key = key_value.trim().to_string();
                    if !api_key.is_empty() {
                        return Ok(Some(api_key));
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn save_api_key(&self, api_key: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)?;
        entry.set_password(api_key)?;
        Ok(())
    }
}
