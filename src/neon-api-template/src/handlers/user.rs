use crate::{
    data_api::DataApi,
    errors::AppError,
    models::{Claims, UserResponse},
    AppState,
};
use axum::{extract::State, response::Json, Extension};

#[axum::debug_handler]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, AppError> {
    let data_api = DataApi::new(
        state.http_client.clone(),
        state.config.neon_data_api_url.clone(),
        None,
    );

    let user = data_api
        .find_user_by_id(&claims.sub, None)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".into()))?;

    Ok(Json(UserResponse::from(user)))
}
