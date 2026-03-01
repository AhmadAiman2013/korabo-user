use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use lambda_http::tracing::log::warn;
use serde_json::json;
use thiserror::Error;
use user_core::UserError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Profile not ready yet")]
    ProfilePending,

    #[error(transparent)]
    Repository(#[from] UserError)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::ProfilePending => (
                StatusCode::ACCEPTED,
                "korabo_user_201",
                "Profile is being set up, please retry shortly".to_string(),
                ),
            AppError::Repository(UserError::ProfileNotFound(id)) => (
                StatusCode::NOT_FOUND,
                "korabo_user_401",
                format!("Profile with id {} not found", id),
                ),
            AppError::Repository(e) => {
                warn!("Repository error: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "korabo_user_404",
                        "An unexpected error occurred".to_string(),
                    )
            }
        };

        (status, Json(json!({
            "code": code,
            "message": message
        }))).into_response()
    }
}