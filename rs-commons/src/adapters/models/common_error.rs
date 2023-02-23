use crate::db::services::DbServiceError;
use derive_more::Display;
use serde_json::{json, Value};

#[derive(Display, Debug)]
#[display(fmt = "({}, {}, {}", description, value, reason)]
pub struct ErrorDefinition {
    pub description: String,
    #[display(fmt = "{:?}", location)]
    pub location: Option<String>,
    pub value: Value,
    pub reason: Value,
}

impl ErrorDefinition {
    pub fn new(description: String, location: String, value: Value) -> Self {
        ErrorDefinition {
            description,
            location: Some(location),
            value,
            reason: json!({}),
        }
    }
    pub fn empty(description: String) -> Self {
        ErrorDefinition {
            description,
            location: None,
            value: Default::default(),
            reason: json!({}),
        }
    }
    pub fn with_reason(description: String, reason: Value) -> Self {
        ErrorDefinition {
            description,
            location: None,
            value: Default::default(),
            reason,
        }
    }
    pub fn as_json(&self) -> Value {
        json!({
            "description": self.description,
            "location": self.location,
            "value": self.value,
            "reason": self.reason
        })
    }

    pub fn from_db(err: &DbServiceError) -> Self {
        match err {
            DbServiceError::NotFound => ErrorDefinition::empty("Not found".to_string()),
            DbServiceError::QueryError(err) => {
                ErrorDefinition::with_reason("DbService error".to_string(), json!({ "error": err }))
            }
        }
    }
}
