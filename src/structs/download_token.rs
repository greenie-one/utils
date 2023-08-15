use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadToken {
    pub url: String,
    pub iat: u64,
    pub exp: u64,
}