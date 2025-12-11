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

#[cfg(test)]
mod tests {
    use super::ApiKeyStorage;
    use std::fs;
    use std::io::Write;

    use tempfile::TempDir;

    fn setup_home_with_config(contents: &str) -> TempDir {
        let temp_dir = TempDir::new().expect("temp dir should be created");
        let config_path = temp_dir.path().join(".wakatime.cfg");
        let mut file = fs::File::create(&config_path).expect("config file should be created");
        writeln!(file, "{}", contents).expect("write config contents");
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }
        temp_dir
    }

    #[test]
    fn load_from_wakatime_cfg_returns_key_when_present() {
        let _temp = setup_home_with_config("api_key = 123456");
        let storage = ApiKeyStorage::new().expect("storage");

        let key = storage.load_from_wakatime_cfg().expect("should load");

        assert_eq!(key, Some("123456".to_string()));
    }

    #[test]
    fn load_from_wakatime_cfg_ignores_missing_file() {
        let temp_dir = TempDir::new().expect("temp dir should be created");
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }
        let storage = ApiKeyStorage::new().expect("storage");

        let key = storage.load_from_wakatime_cfg().expect("should load");

        assert!(key.is_none());
    }

    #[test]
    fn load_api_key_prefers_env_var() {
        let _temp = setup_home_with_config("api_key = from_file");
        unsafe {
            std::env::set_var("WAKATIME_API_KEY", "from_env");
        }

        let storage = ApiKeyStorage::new().expect("storage");

        let key = storage.load_api_key().expect("should load");

        assert_eq!(key, Some("from_env".to_string()));

        unsafe {
            std::env::remove_var("WAKATIME_API_KEY");
        }
    }
}
