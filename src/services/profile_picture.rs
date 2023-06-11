use axum::extract::multipart::Field;
use azure_storage_blobs::prelude::ContainerClient;
use mongodb::Client;
use mongodb::bson::{doc, Document};

use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;

pub async fn set_profile_picture<'a>(
    user_details: TokenClaims,
    db: Client,
    // mut file: Field<'a>,
    // container_client: ContainerClient,
) -> Result<()> {
    let db_name = std::env::var("MONGO_DB_NAME").unwrap();
    println!("DB Name: {}", db_name);
    let profile: Document = db.database(db_name.as_str()).collection("profiles").find_one(doc! {
        "firstName": "Ratnesh"
    }, None).await.unwrap().expect("Profile not found");
    println!("Profile: {:?}", profile);
    Ok(())
}