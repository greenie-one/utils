use mongodb::bson::Document;
use crate::{services::{file_storage::{FileStorageService, StorageEnum}, admin::AdminService}, database::mongo::MongoDB, models::user_nonces::UserNonce, remote::emailer::Emailer};

#[derive(Clone)]
pub struct ProfilePicState {
    pub service: FileStorageService,
}

impl ProfilePicState {
    pub fn new() -> Self {
        Self {
            service: FileStorageService::new("images".into(), StorageEnum::ProfilePicture),
        }
    }
}

#[derive(Clone)]
pub struct DocDepotState {
    pub document_collection: mongodb::Collection<Document>,
    pub nonce_collection: mongodb::Collection<UserNonce>,
}

impl DocDepotState {
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

#[derive(Clone)]
pub struct LeadState {
    pub service: FileStorageService,
    pub emailer: Emailer,
}

impl LeadState {
    pub fn new() -> Self {
        Self {
            service: FileStorageService::new("leads".into(), StorageEnum::Leads),
            emailer: Emailer::new(),
        }
    }
}
    