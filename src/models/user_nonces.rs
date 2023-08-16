use mongodb::bson::{oid::ObjectId, doc, to_document};
use serde::{Deserialize, Serialize};

use crate::database::mongo::MongoDB;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Nonces {
    pub user_id: ObjectId,
    pub nonce: Vec<u8>,
}

impl Nonces {
    pub fn new(user_id: ObjectId, nonce: Vec<u8>) -> Self {
        Self {
            user_id: user_id,
            nonce,
        }
    }

    pub async fn create(&self, db: MongoDB) -> Result<(), mongodb::error::Error> {
        let collection = db.connection.collection::<Nonces>("nonces");
        collection.insert_one(self, None).await?;
        Ok(())
    }

    pub async fn delete(&self, db: MongoDB) -> Result<(), mongodb::error::Error> {
        let collection = db.connection.collection::<Nonces>("nonces");
        collection.delete_one(to_document(self)?, None).await?;
        Ok(())
    }
}
