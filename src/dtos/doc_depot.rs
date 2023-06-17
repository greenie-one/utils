use serde::{Deserialize, Serialize};

use crate::models::documents::DocumentType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentDto {
    pub name: String,

    #[serde(rename = "type")]
    pub doc_type: DocumentType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDocumentDto {
    pub name: Option<String>,

    #[serde(rename = "type")]
    pub doc_type: Option<DocumentType>,
}
