use azure_storage_blobs::prelude::ContainerClient;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};

use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;

use super::file_handling::{delete_file, upload_file_chunked, File};

pub async fn set_profile_picture<'a>(
    user_details: TokenClaims,
    db: Client,
    file: File<'a>,
    container_client: ContainerClient,
) -> Result<String> {
    let db_name = std::env::var("MONGO_DB_NAME").unwrap();
    let mut collection: Collection<Document> = db.database(db_name.as_str()).collection("profiles");

    let profile = get_profile(user_details, &mut collection).await?;
    let url = upload_file_chunked(file, container_client).await?;

    collection
        .update_one(
            profile,
            doc! {
                "$set": {
                    "profilePicture": url.as_str()
                }
            },
            None,
        )
        .await?;

    Ok(url.to_string())
}

pub async fn get_profile(
    user_details: TokenClaims,
    collection: &mut Collection<Document>,
) -> Result<Document> {
    let profile: Document = collection
        .find_one(
            doc! {
                "user": ObjectId::parse_str(user_details.sub.as_str())?
            },
            None,
        )
        .await
        .unwrap()
        .ok_or_else(|| crate::errors::Error::ProfileNotFound)?;

    Ok(profile)
}

pub async fn remove_profile_picture<'a>(
    user_details: TokenClaims,
    db: Client,
    container_client: ContainerClient,
) -> Result<()> {
    let db_name = std::env::var("MONGO_DB_NAME").unwrap();
    let mut collection: Collection<Document> = db.database(db_name.as_str()).collection("profiles");

    let profile = get_profile(user_details, &mut collection).await?;

    let profile_picture = profile.get_str("profilePicture").unwrap();

    let profile_picture = profile_picture.split("/").last().unwrap();

    delete_file(profile_picture, container_client).await?;

    collection
        .update_one(
            profile,
            doc! {
                "$set": {
                    "profilePicture": ""
                }
            },
            None,
        )
        .await?;

    Ok(())
}
