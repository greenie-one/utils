use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatus {
    pub commited: bool,
}