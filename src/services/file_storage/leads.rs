use axum::response::IntoResponse;
use crate::{
    env_config::APP_ENV,
    errors::api_errors::{APIError, APIResult},
    structs::{download_token::DownloadToken, files::File},
    utils::azure::get_container_client,
};

use super::FileStorageService;

#[derive(Clone)]
pub struct Leads();

impl Leads {
    pub fn new() -> FileStorageService<Self> {
        FileStorageService {
            container_client: get_container_client("leads"),
            _phantom: std::marker::PhantomData::<Leads>,
        }
    }

    pub fn from_token(token: String, filename: &str) -> APIResult<FileStorageService<Self>> {
        let private_url = Self::constuct_url(filename);
        let token_url = DownloadToken::validate(token)?.url;
        if private_url != token_url {
            return Err(APIError::BadToken);
        }

        Ok(Leads::new())
    }
    
    pub fn constuct_url(file_name: &str) -> String {
        let env = APP_ENV.as_str();
        let url = match env {
            "production" => format!("https://api.greenie.one/utils/leads/{}", file_name),
            _ => format!("https://dev-api.greenie.one/utils/leads/{}", file_name),
        };
        url
    }
}

impl FileStorageService<Leads> {
    pub async fn generate_signed_download_url(&self, raw_url: String) -> APIResult<String> {
        let token = DownloadToken::new_from_days(raw_url.clone(), 365)?.encode();
        let mut url = url::Url::parse(&raw_url)?;
        url.set_query(Some(&format!("token={}", token)));
        Ok(url.to_string())
    }

    pub async fn upload_file(&mut self, file: File<'_>) -> APIResult<String> {
        let url = self.uploader(file).await?;
        let file_name = url.path_segments().unwrap().last().unwrap();
        Ok(Leads::constuct_url(file_name))
    }

    pub async fn download_file(&self, file_name: String) -> APIResult<impl IntoResponse> {
        self.downloader(file_name).await
    }
}
