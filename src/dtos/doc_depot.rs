use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadDTO {
    pub token: Option<String>,
}