use crate::{
    database::{nonces::NonceCollection, user_documents::UserDocumentsCollection},
    remote::emailer::Emailer,
    services::{admin::AdminService, leads::Leads, profile::Profile, file_storage::FileStorageService},
};

#[derive(Clone)]
pub struct ProfilePicState {
    pub service: FileStorageService<Profile>,
}

impl ProfilePicState {
    pub fn new() -> Self {
        Self {
            service: Profile::new(),
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
    pub service: FileStorageService<Leads>,
    pub emailer: Emailer,
}

impl LeadState {
    pub fn new() -> Self {
        Self {
            service: Leads::new(),
            emailer: Emailer::new(),
        }
    }
}
