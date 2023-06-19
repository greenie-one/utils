use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatus {
    pub commited: bool,
    pub upload_time: DateTime<Utc>,
}