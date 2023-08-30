use mongodb::{Collection, bson::{Document, doc}};

use crate::errors::api_errors::APIResult;
use super::mongo::MongoDB;

#[derive(Clone)]
pub struct UserDocumentsCollection {
    collection: Collection<Document>,
}

impl UserDocumentsCollection {
    pub async fn new() -> Self {
        let db = MongoDB::new().await;
        Self {
            collection: db.connection.collection("documents"),
        }
    }
}

impl UserDocumentsCollection {
    pub async fn exists(&self, priavte_url: String) -> APIResult<bool> {
        let doc = self.collection
            .find_one(doc! {"privateUrl": priavte_url}, None)
            .await?;
        match doc {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}