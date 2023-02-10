use std::collections::HashMap;
use serde_json::Value;
use serde::{Serialize};

pub struct CreateTask {
    pub flow: String,
    pub arguments: Option<HashMap<String, Value>>
}

#[derive(Serialize)]
pub struct TaskDefinition {

}
