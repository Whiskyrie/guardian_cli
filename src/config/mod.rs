use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const DEFAULT_GUARDIAN_URL: &str = "https://guardian.whiskyrie.com.br";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub guardian_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            guardian_url: DEFAULT_GUARDIAN_URL.to_string(),
        }
    }
}

impl Config {
    fn guardian_dir() -> PathBuf {
        dirs_home().join(".guardian")
    }

    fn config_path() -> PathBuf {
        Self::guardian_dir().join("config.toml")
    }

    fn token_path() -> PathBuf {
        Self::guardian_dir().join("token")
    }

    pub fn load() -> Result<Self> {
        let mut config = Self::default();

        // 1. Load ~/.guardian/config.toml (lower priority)
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let file_config: Config = toml::from_str(&content)?;
            if !file_config.guardian_url.is_empty() {
                config.guardian_url = file_config.guardian_url;
            }
        }

        // 2. Load .env from current dir (higher priority, overrides config.toml)
        let _ = dotenvy::from_path_override("./.env");
        if let Ok(url) = std::env::var("GUARDIAN_URL")
            && !url.is_empty()
        {
            config.guardian_url = url;
        }

        Ok(config)
    }

    pub fn load_token() -> Result<Option<String>> {
        let token_path = Self::token_path();
        if token_path.exists() {
            let token = fs::read_to_string(&token_path)?;
            let trimmed = token.trim().to_string();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed))
            }
        } else {
            Ok(None)
        }
    }

    pub fn save_token(token: &str) -> Result<()> {
        let dir = Self::guardian_dir();
        fs::create_dir_all(&dir)?;
        fs::write(Self::token_path(), token)?;
        Ok(())
    }

    pub fn clear_token() -> Result<()> {
        let token_path = Self::token_path();
        if token_path.exists() {
            fs::remove_file(token_path)?;
        }
        Ok(())
    }

    pub fn graphql_url(&self) -> String {
        format!("{}/graphql", self.guardian_url.trim_end_matches('/'))
    }
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}
