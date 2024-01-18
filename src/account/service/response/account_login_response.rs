use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    redis_token: String,
}

impl AccountLoginResponse {
    pub fn new(redis_token: String) -> Self {
        AccountLoginResponse { redis_token }
    }
}