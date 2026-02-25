use serde::{Deserialize, Serialize};

/// FPX bank info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpxBank {
    pub id: String,
    pub name: String,
    pub code: String,
    pub status: String,
}
