use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use jwt::{AuthClaims, JwtPublicKey};
use serde_json::{json, Value};
use user_core::UserRepository;
use crate::error::AppError;

#[derive(Clone)]
pub  struct  AppState {
    pub jwt: JwtPublicKey,
    pub repo: Arc<UserRepository>,
}

impl AsRef<JwtPublicKey> for AppState {
    fn as_ref(&self) -> &JwtPublicKey {
        &self.jwt
    }
}

// GET /user/health
pub async fn health_check() -> Json<Value> {
    let health = true;
    match health {
        true => Json(json!({ "status": "healthy" })),
        false => Json(json!({ "status": "unhealthy" })),
    }
}

// GET /user
pub async fn get_user(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
) -> Result<Json<Value>, AppError> {
    match state.repo.get_profile(&claims.sub).await? {
        None => Err(AppError::ProfilePending),
        Some(p) => Ok(Json(json!({
            "user_id": p.user_id,
            "email": p.email,
            "name": p.name,
            "interests": p.interest,
            "study_preference": p.study_preference,
            "privacy": p.privacy,
            "created_at": p.created_at,
        }))),
    }
}