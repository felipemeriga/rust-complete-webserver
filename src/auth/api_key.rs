use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ApiKeyData {
    pub api_key: String,
    pub enable_api_key: bool,
}
