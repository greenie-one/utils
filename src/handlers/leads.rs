use crate::errors::api_errors::{APIError, APIResult};
use crate::state::app_state::LeadState;
use crate::structs::download_token::DownloadToken;
use crate::structs::files::File;
use crate::structs::token_claims::TokenClaims;

use axum::extract::Multipart;
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn upload(
    State(mut state): State<LeadState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    if !user_details.roles.contains(&"hr".to_owned()) {
        return Err(APIError::Unauthorized);
    }

    let field = multipart
        .next_field()
        .await?
        .ok_or_else(|| APIError::NoFileAttached)?;
    let mut file = File::try_from(field)?;

    file.validate_csv()?;
    file.name = format!("{}-{}", user_details.sub, file.name);
    let url = state.service.upload_file(file).await?;
    
    let token = DownloadToken::new_from_days(url.clone(), 365)?.encode();
    let mut url = url::Url::parse(&url)?;
    url.set_query(Some(&format!("token={}", token)));

    tokio::spawn(async move {
        let res = state.emailer.notify_bulk_upload(user_details, url.as_str()).await;
        if let Err(err) = res {
            tracing::error!("{}", err);
        }
    });

    Ok(Json(json!({
        "message": "File uploaded successfully",
    })))
}
