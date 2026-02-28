use std::collections::HashMap;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use serde_dynamo::{from_item, to_item};
use crate::error::UserError;
use crate::models::{PrivacySettings, UserProfile, UserRegisteredEvent};

pub struct UserRepository {
    client: Client,
    table_name: String,
}

impl UserRepository {
    pub fn new(client: Client, table_name: String) -> Self {
        Self { client, table_name }
    }

    pub async fn create_default_profile(
        &self,
        event: &UserRegisteredEvent
    ) -> Result<(), UserError> {
        let profile = UserProfile {
            user_id: event.user_id.clone(),
            email: event.email.clone(),
            name: None,
            interest: vec![],
            study_preference: None,
            privacy: PrivacySettings::default(),
            created_at: Utc::now(),
        };

        let mut item: HashMap<String, AttributeValue> = to_item(profile)?;
        item.insert("PK".to_string(), AttributeValue::S(format!("USER#{}", event.user_id)));
        item.insert("SK".to_string(), AttributeValue::S("PROFILE".to_string()));

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(PK)")
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        Ok(())
    }

    pub async fn get_profile(
        &self,
        user_id: &str,
    ) -> Result<Option<UserProfile>, UserError> {
        let result = self.client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(format!("USER#{}", user_id)))
            .key("SK", AttributeValue::S("PROFILE".to_string()))
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        match result.item {
            None => Ok(None),
            Some(item) => Ok(Some(from_item(item)?))
        }
    }
}