use crate::{
    auth::{create_jwt, hash_password, verify_password},
    data_api::DataApi,
    errors::AppError,
    models::{AuthResponse, LoginRequest, SignupRequest, UserResponse},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::json;
use uuid::Uuid;

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    if payload.email.is_empty() || !payload.email.contains('@') {
        return Err(AppError::Validation("valid email required".into()));
    }
    if payload.password.len() < 8 {
        return Err(AppError::Validation(
            "password must be at least 8 characters".into(),
        ));
    }
    if payload.first_name.trim().is_empty() || payload.last_name.trim().is_empty() {
        return Err(AppError::Validation("first and last name required".into()));
    }

    let data_api = DataApi::new(
        state.http_client.clone(),
        state.config.neon_data_api_url.clone(),
        None,
    );

    if data_api
        .find_user_by_email(&payload.email, None)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("user already exists".into()));
    }

    let password_hash = hash_password(&payload.password)?;
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();

    let new_user = json!({
        "id": user_id.to_string(),
        "email": payload.email,
        "password_hash": password_hash,
        "first_name": payload.first_name,
        "last_name": payload.last_name,
        "created_at": now,
        "updated_at": now,
    });

    let user = data_api.insert_user(&new_user, None).await?;
    let token = create_jwt(&user.id.to_string(), &user.email, &state.config)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: UserResponse::from(user),
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::Validation("email and password required".into()));
    }

    let data_api = DataApi::new(
        state.http_client.clone(),
        state.config.neon_data_api_url.clone(),
        None,
    );

    let user = data_api
        .find_user_by_email(&payload.email, None)
        .await?
        .ok_or_else(|| AppError::Authentication("invalid email or password".into()))?;

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Authentication("invalid email or password".into()));
    }

    let token = create_jwt(&user.id.to_string(), &user.email, &state.config)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}
