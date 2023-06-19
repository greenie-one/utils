use crate::{dtos::token_claims::TokenClaims, errors::{Result, Error}};

use super::file_handling::File;
use axum::extract::multipart::Field;

pub fn validate_image_field<'a>(field: Field<'a>, user_details: &TokenClaims) -> Result<File<'a>> {
    let file_name = field.file_name().ok_or_else(|| Error::InvalidFileName)?;
    let content_type = field
        .content_type()
        .ok_or_else(|| Error::InvalidContentType)?;

    if !content_type.starts_with("image/") {
        return Err(Error::InvalidContentType);
    }

    if !file_name.ends_with(".jpg") && !file_name.ends_with(".jpeg") && !file_name.ends_with(".png")
    {
        return Err(Error::InavlidFileExtension);
    }

    let file_extension = file_name.split('.').last().unwrap();
    let file_name = format!("{}.{}", user_details.sub, file_extension);

    Ok(File {
        name: file_name.to_string(),
        content_type: content_type.to_string(),
        field,
    })
}

pub fn validate_pdf_field(field: Field) -> Result<File> {
    let file_name = field.file_name().ok_or_else(|| Error::InvalidFileName)?;
    let content_type = field
        .content_type()
        .ok_or_else(|| Error::InvalidContentType)?;

    if !content_type.starts_with("application/pdf") {
        return Err(Error::InvalidContentType);
    }

    if !file_name.ends_with(".pdf") {
        return Err(Error::InavlidFileExtension);
    }

    Ok(File {
        name: file_name.to_string(),
        content_type: content_type.to_string(),
        field,
    })
}
