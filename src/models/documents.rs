use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    Work,
    Certificate,
    Marksheet,
    Tax,
    Education,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "_id")]
    pub id: mongodb::bson::oid::ObjectId,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub type_: DocumentType,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "user")]
    pub user: mongodb::bson::oid::ObjectId,

    #[serde(rename = "createdAt")]
    pub created_at: BsonDateTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: BsonDateTime,
}
