use std::sync::Arc;
use lambda_runtime::{run, service_fn, tracing, Error};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;

mod event_handler;
use event_handler::function_handler;
use user_core::UserRepository;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let table_name = String::from("korabo_user");
    let repo = Arc::new(UserRepository::new(client, table_name));
    
    run(service_fn(move |event| {
        let repo = repo.clone();
        async move { function_handler(event, repo).await }
    }))
        .await

}
