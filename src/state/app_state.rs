use crate::{
    database::{nonces::NonceCollection, user_documents::UserDocumentsCollection},
    remote::emailer::Emailer,
    services::{admin::AdminService, leads::LeadsService, profile::ProfileService},
};

#[derive(Clone)]
pub struct ProfilePicState {
    pub service: ProfileService,
}

impl ProfilePicState {
    pub fn new() -> Self {
        Self {
            service: ProfileService::new(),
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
    pub service: LeadsService,
    pub emailer: Emailer,
}

impl LeadState {
    pub fn new() -> Self {
        Self {
            service: LeadsService::new(),
            emailer: Emailer::new(),
        }
    }
}
