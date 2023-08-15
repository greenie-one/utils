use mongodb::bson::Document;

use crate::{services::{doc_depot::DocDepotService, admin::AdminService}, database::mongo::MongoDB};

#[derive(Clone)]
pub struct DocDepotState {
    pub service: DocDepotService,
}

#[derive(Clone)]
pub struct UplaodState {
    pub document_collection: mongodb::Collection<Document>
}

impl UplaodState {
    pub async fn new() -> Self {
        let db = MongoDB::new().await;
        let document_collection = db.connection.collection("documents");
        Self {
            document_collection
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
    