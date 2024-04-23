use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Auth0Data {
    pub audience: String,
    pub domain: String,
}
