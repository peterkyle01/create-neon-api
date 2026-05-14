use crate::{auth::verify_jwt, errors::AppError, AppState};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = request.uri().path();
    if matches!(path, "/health" | "/signup" | "/login") {
        return Ok(next.run(request).await);
    }

    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Authentication("missing authorization header".into()))?;

    let claims = verify_jwt(token, &state.config)?;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
