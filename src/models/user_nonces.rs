use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};
use tracing::log::error;

use crate::{
    errors::api_errors::{APIError, APIResult},
    utils::encrypt::generate_nonce,
};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct UserNonce {
    pub user_id: ObjectId,
    pub nonce: Vec<u8>,
}

impl UserNonce {
    pub fn new(user_id: ObjectId, nonce: Vec<u8>) -> Self {
        Self { user_id, nonce }
    }

    pub async fn create_or_fetch(
        user_id: String,
        collection: Collection<UserNonce>,
    ) -> APIResult<UserNonce> {
        let nonce = collection
            .find_one(doc! {"user_id": user_id.clone()}, None)
            .await?;
        match nonce {
            Some(nonce) => Ok(nonce),
            None => {
                let nonce = UserNonce::new(ObjectId::from_str(&user_id)?, generate_nonce());
                nonce.create(collection).await?;
                Ok(nonce)
            }
        }
    }

    pub async fn fetch(user_id: String, collection: Collection<UserNonce>) -> APIResult<UserNonce> {
        let nonce = collection
            .find_one(doc! {"user_id": user_id.clone()}, None)
            .await?;
        match nonce {
            Some(nonce) => Ok(nonce),
            None => {
                error!("Nonce not found for user {}", user_id);
                Err(APIError::FileNotFound)?
            }
        }
    }

    pub async fn create(&self, collection: Collection<UserNonce>) -> APIResult<()> {
        collection.insert_one(self, None).await?;
        Ok(())
    }
}
