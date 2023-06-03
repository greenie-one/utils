use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SendMailDto {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub enum MailBodyType {
    String(String),
    Html(String),
}