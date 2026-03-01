use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use jwt::{AuthClaims, JwtPublicKey};
use serde_json::{json, Value};
use user_core::{AddCourseRequest, UpdateProfileRequest, UserRepository};
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
    let (profile, courses) = tokio::join!(
        state.repo.get_profile(&claims.sub),
        state.repo.get_courses(&claims.sub),
    );

    match profile? {
        None => Err(AppError::ProfilePending),
        Some(p) => Ok(Json(json!({
            "user_id": p.user_id,
            "email": p.email,
            "name": p.name,
            "courses": courses?,
            "interests": p.interests,
            "study_preference": p.study_preferences,
            "privacy": p.privacy,
        }))),
    }
}

// GET /user/:userId
pub async fn get_public_profile(
    State(state): State<AppState>,
    AuthClaims(_claims): AuthClaims,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let (profile, courses) = tokio::join!(
        state.repo.get_profile(&user_id),
        state.repo.get_courses(&user_id),
    );

    let profile = profile?
        .ok_or_else(|| AppError::ProfilePending)?;

    let mut body = json!({
        "user_id": profile.user_id,
        "name": profile.name,
        "interests": profile.interests,
        "studyPreferences": profile.study_preferences,
    });

    if profile.privacy.show_courses {
        body["courses"] = json!(courses?);
    }

    Ok(Json(body))
}

// POST /user
pub async fn update_me(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<StatusCode, AppError> {
    state.repo.update_profile(&claims.sub, &body).await?;
    Ok(StatusCode::NO_CONTENT)
}

// POST /user/courses
pub async fn add_course(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
    Json(body): Json<AddCourseRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let course_id = body.normalized_course_id();
    state.repo.add_course(&claims.sub, &course_id).await?;

    Ok((StatusCode::CREATED, Json(json!({
        "code": "korabo_user_2021",
        "course_id": course_id,
    }))))
}

// Delete /me/course/:courseId
pub async fn remove_course(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
    Path(course_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let course_id = course_id.trim().to_uppercase().replace(" ", "");
    state.repo.remove_course(&claims.sub, &course_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
