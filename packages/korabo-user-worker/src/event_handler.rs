use std::sync::Arc;
use lambda_runtime::{Error, LambdaEvent};
use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::tracing::log::error;
use serde_json::from_str;
use user_core::{UserRegisteredEvent, UserRepository};

pub(crate)async fn function_handler(
    event: LambdaEvent<SqsEvent>,
    repo: Arc<UserRepository>
) -> Result<(), Error> {
    for record in event.payload.records {
        let body= match record.body {
            Some(b) => b,
            None => {
                error!("Received SQS record with no body, message_id: {:?}", record.message_id);
                continue;
            }
        };

        let user_event: UserRegisteredEvent = match from_str(&body) {
            Ok(e) => e,
            Err(err) => {

                error!("Failed to deserialize message: {:?} body: {}", err, body);
                continue;
            }
        };

        repo.create_default_profile(&user_event).await?;
    }
    Ok(())
}


