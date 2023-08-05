use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatus {
    pub commited: bool,
    pub upload_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDeleteRequest {
    pub file_name: String,
    pub container_name: String,
}