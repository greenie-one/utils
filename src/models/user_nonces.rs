use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct UserNonce {
    pub user_id: ObjectId,
    pub nonce: Vec<u8>,
}

impl UserNonce {
    pub fn new(user_id: ObjectId, nonce: Vec<u8>) -> Self {
        Self { user_id, nonce }
    }
}
