use crate::{services::{file_storage::{FileStorageService, StorageEnum}, admin::AdminService}, database::{nonces::NonceCollection, user_documents::UserDocumentsCollection}, remote::emailer::Emailer};

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
    pub document_collection: UserDocumentsCollection,
    pub nonce_collection: NonceCollection,
}

impl DocDepotState {
    pub async fn new() -> Self {
        Self {
            document_collection: UserDocumentsCollection::new().await,
            nonce_collection: NonceCollection::new().await,
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
    