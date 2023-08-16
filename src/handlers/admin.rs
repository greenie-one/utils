use crate::dtos::admin::CreateUser;
use crate::errors::api_errors::APIResult;
use crate::state::app_state::AdminState;
use crate::{errors::api_errors::APIError, structs::token_claims::TokenClaims};
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

pub async fn create_account(
    State(state): State<AdminState>,
    user_details: TokenClaims,
    Json(create_user): Json<CreateUser>,
) -> APIResult<Json<Value>> {
    if !(user_details.roles.contains(&"admin".to_owned())) {
        return Err(APIError::Unauthorized);
    }

    if !(create_user.roles.contains(&"admin".to_owned())) {
        return Err(APIError::Unauthorized);
    }

    state.service.create_account(create_user).await?;

    Ok(Json(json!({
        "message": "HR profile created successfully"
    })))
}
