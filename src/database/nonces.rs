use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use tracing::log::error;

use crate::{
    errors::api_errors::{APIError, APIResult},
    models::user_nonces::UserNonce,
    utils::encrypt::generate_nonce,
};

use super::mongo::MongoDB;

#[derive(Clone)]
pub struct NonceCollection {
    collection: Collection<UserNonce>,
}

impl NonceCollection {
    pub async fn new() -> Self {
        let db = MongoDB::new().await;
        Self {
            collection: db.connection.collection("nonces"),
        }
    }
}

impl NonceCollection {
    pub async fn create_or_fetch(&self, user_id: String) -> APIResult<UserNonce> {
        let nonce = self
            .collection
            .find_one(doc! {"user_id": ObjectId::from_str(&user_id)?}, None)
            .await?;
        match nonce {
            Some(nonce) => Ok(nonce),
            None => {
                let nonce = UserNonce::new(ObjectId::from_str(&user_id)?, generate_nonce());
                self.collection.insert_one(nonce.clone(), None).await?;
                Ok(nonce)
            }
        }
    }

    pub async fn fetch(&self, user_id: String) -> APIResult<UserNonce> {
        let nonce = self
            .collection
            .find_one(doc! {"user_id": ObjectId::from_str(&user_id)?}, None)
            .await?;
        match nonce {
            Some(nonce) => Ok(nonce),
            None => {
                error!("Nonce not found for user {}", user_id);
                Err(APIError::FileNotFound)?
            }
        }
    }
}
