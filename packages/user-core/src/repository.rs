use std::collections::HashMap;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use serde_dynamo::{from_item, from_items, to_item};
use crate::error::UserError;
use crate::models::{UserProfile, UserRegisteredEvent};
use crate::{CourseItem, CourseRecord, PrivacySettings, ProfileItem, UpdateProfileRequest};

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
        let item = ProfileItem {
            pk: format!("USER#{}", event.user_id),
            sk: "PROFILE".to_string(),
            profile: UserProfile {
                user_id: event.user_id.clone(),
                email: event.email.clone(),
                name: None,
                interests: vec![],
                study_preferences: None,
                privacy: PrivacySettings::default(),
                created_at: Utc::now(),
            },
        };

        let dynamo_item: HashMap<String, AttributeValue> = to_item(item)?;

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(dynamo_item))
            .condition_expression("attribute_not_exists(PK)")
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        Ok(())
    }

    pub async fn get_courses(
        &self,
        user_id: &str,
    ) -> Result<Vec<String>, UserError> {
        let result = self.client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("PK = :pk AND begins_with(SK, :prefix)")
            .expression_attribute_values(":pk", AttributeValue::S(format!("USER#{}", user_id)))
            .expression_attribute_values(":prefix", AttributeValue::S("COURSE#".to_string()))
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        let raw: Vec<_> = result.items.into_iter().flatten().collect();
        let items: Vec<CourseItem> = from_items(raw)?;
        Ok(items.into_iter().map(|c| c.course_id).collect())
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

    pub async fn update_profile(
        &self,
        user_id: &str,
        req: &UpdateProfileRequest,
    ) -> Result<(), UserError> {
        let mut update_expr = Vec::new();
        let mut expr_attr_names = HashMap::new();
        let mut expr_attr_values = HashMap::new();

        if let Some(name) = &req.name {
            update_expr.push("#n = :name");
            expr_attr_names.insert("#n".to_string(), "name".to_string());
            expr_attr_values.insert(":name".to_string(), AttributeValue::S(name.clone()));
        }

        if let Some(interests) = &req.interest {
            update_expr.push("#i = :interest");
            expr_attr_names.insert("#i".to_string(), "interest".to_string());
            let list = interests.iter().map(|s| { AttributeValue::S(s.clone()) }).collect();
            expr_attr_values.insert(":interest".to_string(), AttributeValue::L(list));
        }

        if let Some(prefs) = &req.study_preferences {
            let prefs_items: HashMap<String, AttributeValue> = to_item(prefs)?;
            update_expr.push("#sp = :study_preferences");
            expr_attr_names.insert("#sp".to_string(), "study_preferences".to_string());
            expr_attr_values.insert(":study_preferences".to_string(), AttributeValue::M(prefs_items));
        }

        if update_expr.is_empty() {
            return Ok(());
        }

        self.client
            .update_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(format!("USER#{}", user_id)))
            .key("SK", AttributeValue::S("PROFILE".to_string()))
            .update_expression(format!("SET {}", update_expr.join(", ")))
            .set_expression_attribute_names(Some(expr_attr_names))
            .set_expression_attribute_values(Some(expr_attr_values))
            .condition_expression("attribute_exists(PK)")
            .send()
            .await
            .map_err(|e|e.into_service_error())?;

        Ok(())
    }

    pub async fn add_course(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<(), UserError> {
        let record = CourseRecord {
            pk: format!("USER#{}", user_id),
            sk: format!("COURSE#{}", course_id),
            course_id: course_id.to_string(),
            added_at: Utc::now(),
        };

        let dynamo_item: HashMap<String, AttributeValue> = to_item(record)?;

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(dynamo_item))
            .condition_expression("attribute_not_exists(SK)")
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        Ok(())
    }

    pub async fn remove_course(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<(), UserError> {
        self.client
            .delete_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(format!("USER#{}", user_id)))
            .key("SK", AttributeValue::S(format!("COURSE#{}", course_id)))
            .condition_expression("attribute_exists(SK)")
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        Ok(())
    }
}