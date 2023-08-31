pub mod doc_depot;
pub mod leads;
pub mod profile;

use crate::errors::api_errors::{APIError, APIResult};
use crate::structs::files::File;
use axum::body::StreamBody;
use axum::http::header;
use axum::response::IntoResponse;

use azure_storage_blobs::blob::operations::GetBlobResponse;

use azure_storage_blobs::prelude::ContainerClient;
use futures_util::StreamExt;
use url::Url;

#[derive(Clone)]
pub struct FileStorageService<Type> {
    pub container_client: ContainerClient,
    pub _phantom: std::marker::PhantomData<Type>,
}

impl<Type> FileStorageService<Type> {
    pub(self) async fn uploader(&mut self, file: File<'_>) -> APIResult<Url> {
        let file_name = file.name.as_str();
        let content_type = file.content_type.clone();

        let blob_client = self.container_client.blob_client(file_name);

        blob_client
            .put_block_blob(file.field.bytes().await?)
            .content_type(content_type)
            .await?;

        let url = blob_client.url()?;
        Ok(url)
    }

    pub(self) async fn uploader_encrypted(&mut self, file: File<'_>, nonce: Vec<u8>) -> APIResult<Url> {
        let file_name = file.name.as_str();
        let content_type = file.content_type.clone();

        let blob_client = self.container_client.blob_client(file_name);

        blob_client
            .put_block_blob(file.encrypt(nonce).await?)
            .content_type(content_type)
            .await?;

        let url = blob_client.url()?;
        Ok(url)
    }

    async fn fetch_file(&self, file_name: String) -> APIResult<GetBlobResponse> {
        let blob_client = self.container_client.blob_client(file_name);
        if blob_client.exists().await? == false {
            return Err(APIError::FileNotFound);
        }

        let mut data = blob_client.get().into_stream();
        let blob = data
            .next()
            .await
            .ok_or_else(|| APIError::InternalServerError("No data found".to_string()))??;

        Ok(blob)
    }

    pub(self) async fn downloader(&self, file_name: String) -> APIResult<impl IntoResponse> {
        let blob = self.fetch_file(file_name).await?;
        let content_type = blob.blob.properties.content_type;
        let file_name = blob.blob.name;
        let stream_body = StreamBody::new(blob.data);
        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("form-data; name=\"file\"; filename=\"{}\"", file_name),
            ),
        ];
        Ok((headers, stream_body))
    }

    pub(self) async fn downloader_decrypted(
        &self,
        file_name: String,
        nonce: Vec<u8>,
    ) -> APIResult<impl IntoResponse> {
        let blob = self.fetch_file(file_name).await?;

        let content_type = blob.blob.properties.content_type;
        let file_name = blob.blob.name;
        let data = blob.data.collect().await?.to_vec();

        let body = File::decrypt(nonce, data)?;

        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("form-data; name=\"file\"; filename=\"{}\"", file_name),
            ),
        ];
        Ok((headers, body))
    }
}
