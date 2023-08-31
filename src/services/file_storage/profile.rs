use crate::{structs::files::File, errors::api_errors::APIResult, utils::azure::get_container_client};

use super::FileStorageService;

#[derive(Clone)]
pub struct Profile();

impl Profile {
    pub fn new() -> FileStorageService<Self> {
        FileStorageService {
            container_client: get_container_client("images"),
            _phantom: std::marker::PhantomData::<Profile>,
        }
    }
}

impl FileStorageService<Profile> {
    pub async fn upload_file(&mut self, file: File<'_>) -> APIResult<String> {
        let url = self.uploader(file).await?;
        Ok(url.to_string())
    }
}   