use aws_sdk_dynamodb::Error as DynamoError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use serde_dynamo::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("DynamoDB error: {0}")]
    DynamoDB(#[from] DynamoError),

    #[error("DynamoDB put item error: {0}")]
    PutItem(#[from] PutItemError),

    #[error("DynamoDB get item error: {0}")]
    GetItem(#[from] GetItemError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] Error),

    #[error("Profile not found for user: {0}")]
    ProfileNotFound(String),
}