use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application config loaded from file + env vars
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    #[serde(default)]
    pub sandbox: Option<bool>,
    #[serde(default)]
    pub api_version: Option<String>,
}

impl AppConfig {
    /// Load config from ~/.bayarcash/config.toml, then overlay env vars.
    /// CLI flags are applied by the caller after this.
    pub fn load() -> Self {
        let mut config = Self::from_file().unwrap_or_default();
        config.overlay_env();
        config
    }

    /// Read from config file if it exists
    fn from_file() -> Option<Self> {
        let path = Self::config_path()?;
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    /// Overlay environment variables (higher priority than file)
    fn overlay_env(&mut self) {
        if let Ok(v) = std::env::var("BAYARCASH_TOKEN") {
            self.token = Some(v);
        }
        if let Ok(v) = std::env::var("BAYARCASH_SECRET_KEY") {
            self.secret_key = Some(v);
        }
        if let Ok(v) = std::env::var("BAYARCASH_SANDBOX") {
            self.sandbox = Some(v == "true" || v == "1");
        }
        if let Ok(v) = std::env::var("BAYARCASH_API_VERSION") {
            self.api_version = Some(v);
        }
    }

    /// Get the config file path: ~/.bayarcash/config.toml
    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".bayarcash").join("config.toml"))
    }

    /// Get token or error
    pub fn require_token(&self) -> Result<&str, String> {
        self.token.as_deref().ok_or_else(|| {
            "No API token. Set BAYARCASH_TOKEN env var or add to ~/.bayarcash/config.toml".to_string()
        })
    }

    /// Get secret key or error
    pub fn require_secret_key(&self) -> Result<&str, String> {
        self.secret_key.as_deref().ok_or_else(|| {
            "No secret key. Set BAYARCASH_SECRET_KEY env var or add to ~/.bayarcash/config.toml".to_string()
        })
    }

    /// Resolve sandbox flag (default: false)
    pub fn is_sandbox(&self) -> bool {
        self.sandbox.unwrap_or(false)
    }

    /// Resolve API version (default: "v2")
    pub fn resolved_api_version(&self) -> &str {
        self.api_version.as_deref().unwrap_or("v2")
    }

    /// Build a Bayarcash client from this config
    pub fn build_client(&self) -> Result<crate::Bayarcash, crate::BayarcashError> {
        let token = self.require_token().map_err(crate::BayarcashError::Other)?;
        let api_version = match self.resolved_api_version() {
            "v3" => crate::ApiVersion::V3,
            _ => crate::ApiVersion::V2,
        };
        crate::Bayarcash::builder(token)
            .sandbox(self.is_sandbox())
            .api_version(api_version)
            .build()
    }
}
