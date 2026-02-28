use axum::Json;
use jwt::{AuthClaims, JwtPublicKey};
use serde_json::{json, Value};

#[derive(Clone)]
pub  struct  AppState {
    pub jwt: JwtPublicKey
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
    AuthClaims(claims): AuthClaims,
) -> Json<Value> {

    let user_id = claims.sub;
    Json(json!({
        "msg": "successfully",
        "user_id": user_id,
    }))
}