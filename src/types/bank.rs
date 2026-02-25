use serde::Deserialize;

/// FPX bank info
#[derive(Debug, Clone, Deserialize)]
pub struct FpxBank {
    pub id: String,
    pub name: String,
    pub code: String,
    pub status: String,
}
