use serde::Deserialize;

/// Portal info
#[derive(Debug, Clone, Deserialize)]
pub struct Portal {
    pub id: String,
    pub portal_key: String,
    pub name: String,
    #[serde(default)]
    pub payment_channels: Vec<PaymentChannelInfo>,
}

/// Payment channel details within a portal
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentChannelInfo {
    pub id: u8,
    pub name: String,
    pub code: String,
    pub enabled: bool,
}
