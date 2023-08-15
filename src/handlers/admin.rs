use crate::dtos::admin::CreateUser;
use crate::errors::api_errors::APIResult;
use crate::state::app_state::AdminState;
use crate::{errors::api_errors::APIError, structs::token_claims::TokenClaims};
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

pub async fn create_hr_profile(
    State(state): State<AdminState>,
    user_details: TokenClaims,
    Json(create_user): Json<CreateUser>,
) -> APIResult<Json<Value>> {
    if !(user_details.roles.into_iter().any(|role| role == "admin")) {
        return Err(APIError::Unauthorized);
    }

    state.service.create_hr_profile(create_user).await?;

    Ok(Json(json!({
        "message": "HR profile created successfully"
    })))
}
