use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum DocumentType {
    #[serde(rename = "work")]
    WORK,
    #[serde(rename = "certificate")]
    CERTIFICATE,
    #[serde(rename = "marksheet")]
    MARKSHEET,
    #[serde(rename = "tax")]
    TAX,
    #[serde(rename = "education")]
    EDUCATION,
    #[serde(rename = "other")]
    OTHER,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocQuery {
    pub doc_type: DocumentType,
}
