use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}