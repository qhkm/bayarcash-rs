use crate::types::ApiVersion;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BayarcashConfig {
    pub token: String,
    pub sandbox: bool,
    pub api_version: ApiVersion,
    pub timeout: Duration,
}

impl BayarcashConfig {
    pub fn base_url(&self) -> &str {
        match (self.api_version, self.sandbox) {
            (ApiVersion::V3, true) => "https://api.console.bayarcash-sandbox.com/v3",
            (ApiVersion::V3, false) => "https://api.console.bayar.cash/v3",
            (ApiVersion::V2, true) => "https://console.bayarcash-sandbox.com/api/v2",
            (ApiVersion::V2, false) => "https://console.bayar.cash/api/v2",
        }
    }

    pub fn manual_transfer_base_url(&self) -> &str {
        if self.sandbox {
            "https://console.bayarcash-sandbox.com/api"
        } else {
            "https://console.bayar.cash/api"
        }
    }
}
