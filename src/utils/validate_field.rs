use crate::structs::files::File;
use crate::structs::token_claims::TokenClaims;
use crate::errors::api_errors::APIResult;
use crate::errors::api_errors::APIError;

use axum::extract::multipart::Field;

pub fn validate_image_field<'a>(field: Field<'a>, user_details: &TokenClaims) -> APIResult<File<'a>> {
    let file_name = field.file_name().ok_or_else(|| APIError::InvalidFileName)?;
    let content_type = field
        .content_type()
        .ok_or_else(|| APIError::InvalidContentType)?;

    if !content_type.starts_with("image/") {
        return Err(APIError::InvalidContentType);
    }

    if !file_name.ends_with(".jpg") && !file_name.ends_with(".jpeg") && !file_name.ends_with(".png")
    {
        return Err(APIError::InavlidFileExtension);
    }

    let file_extension = file_name.split('.').last().unwrap();
    let file_name = format!("{}.{}", user_details.sub, file_extension);

    Ok(File {
        name: file_name.to_string(),
        content_type: content_type.to_string(),
        field,
    })
}

pub fn validate_pdf_field(field: Field) -> APIResult<File> {
    let file_name = field.file_name().ok_or_else(|| APIError::InvalidFileName)?;
    let content_type = field
        .content_type()
        .ok_or_else(|| APIError::InvalidContentType)?;

    if !content_type.starts_with("application/pdf") {
        return Err(APIError::InvalidContentType);
    }

    if !file_name.ends_with(".pdf") {
        return Err(APIError::InavlidFileExtension);
    }

    Ok(File {
        name: file_name.to_string(),
        content_type: content_type.to_string(),
        field,
    })
}
