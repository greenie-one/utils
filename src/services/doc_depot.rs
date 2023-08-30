use azure_storage_blobs::prelude::ContainerClient;


use crate::{
    env_config::APP_ENV,
    errors::api_errors::{APIError, APIResult}, database::user_documents::UserDocumentsCollection,
};

#[derive(Clone)]
pub struct DocDepotService {}
impl DocDepotService {
    pub fn constuct_url(container_name: String, file_name: String) -> String {
        let env = APP_ENV.as_str();
        let url = match env {
            "production" => format!(
                "https://api.greenie.one/utils/doc_depot/{}/{}",
                container_name, file_name
            ),
            _ => format!(
                "https://dev-api.greenie.one/utils/doc_depot/{}/{}",
                container_name, file_name
            ),
        };
        url
    }
}

impl DocDepotService {
    pub async fn check_doc_exists(
        container_client: &ContainerClient,
        file_name: String,
        document_collection: UserDocumentsCollection,
    ) -> APIResult<bool> {
        let container_name = container_client.container_name();
        let blob_client = container_client.blob_client(file_name.clone());
        let url = Self::constuct_url(container_name.to_string(), file_name.to_string());
        if blob_client.exists().await? {
            let doc_exists = document_collection
                .exists(url).await?;
            if doc_exists {
                Err(APIError::FileAlreadyExists)?
            }
        }
        Ok(false)
    }
}
