use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserModel {
    pub _id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "mobileNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    pub password: Option<String>,
    pub roles: Vec<String>,
}