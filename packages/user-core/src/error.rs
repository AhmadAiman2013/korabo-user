use aws_sdk_dynamodb::Error as DynamoError;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::update_item::UpdateItemError;
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
    
    #[error("Query error: {0}")]
    QueryError(#[from] QueryError),
    
    #[error("Update error: {0}")]
    UpdateError(#[from] UpdateItemError),
    
    #[error("Delete error: {0}")]
    DeleteError(#[from] DeleteItemError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] Error),

    #[error("Profile not found for user: {0}")]
    ProfileNotFound(String),
}