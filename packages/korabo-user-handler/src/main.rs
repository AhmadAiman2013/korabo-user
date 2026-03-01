use std::env::{set_var, var};
use std::sync::Arc;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use axum::Router;
use axum::routing::get;
use lambda_http::{run, tracing, Error};
mod user_handler;
mod error;

use jwt::JwtPublicKey;
use user_core::UserRepository;
use crate::user_handler::{get_user, health_check, AppState};

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    tracing::init_default_subscriber();

    let jwt = JwtPublicKey::from_jwks_file(
        var("JWT_ISSUER").expect("JWT_ISSUER must be set"),
        var("JWT_AUDIENCE").expect("JWT_AUDIENCE must be set"),
    ).expect("Failed to load JWKS");

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let table_name = String::from("korabo_user");
    let repo = Arc::new(UserRepository::new(client, table_name));

    let state = AppState{ jwt, repo };

    let app = Router::new().nest(
        "/user",
        Router::new()
            .route("/health", get(health_check))
            .route("/user", get(get_user))
            .with_state(state)
    );

    run(app).await

}
