use crate::{structs::files::File, errors::api_errors::APIResult};

use super::file_storage::FileStorageService;

#[derive(Clone)]
pub struct ProfileService {
    file_storage: FileStorageService,
}

impl ProfileService {
    pub fn new() -> Self {
        Self {
            file_storage: FileStorageService::new("images"),
        }
    }

    pub async fn upload_file(&mut self, file: File<'_>) -> APIResult<String> {
        let url = self.file_storage.upload_file(file).await?;
        Ok(url.to_string())
    }
}   