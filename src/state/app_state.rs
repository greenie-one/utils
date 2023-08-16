use mongodb::bson::Document;

use crate::{services::{file_storage::FileStorageService, admin::AdminService}, database::mongo::MongoDB, models::user_nonces::Nonces};

#[derive(Clone)]
pub struct FileStorageState {
    pub service: FileStorageService,
}

#[derive(Clone)]
pub struct UplaodState {
    pub document_collection: mongodb::Collection<Document>,
    pub nonce_collection: mongodb::Collection<Nonces>,
}

impl UplaodState {
    pub async fn new() -> Self {
        let db = MongoDB::new().await;
        Self {
            document_collection: db.connection.collection("documents"),
            nonce_collection: db.connection.collection("nonces"),
        }
    }
}

#[derive(Clone)]
pub struct AdminState {
    pub service: AdminService,
}

impl AdminState {
    pub async fn new() -> Self {
        Self {
            service: AdminService::new().await,
        }
    }
}
    