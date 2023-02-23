use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, NaiveDate, Utc};
use deadpool_postgres::Transaction;
use log::info;
use rhai::{Engine, Scope};
use serde_json::{json, Value};
use serde_json::Value::Bool;
use tokio::sync::Mutex;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::data_type::DataType;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::db::services::DbServices;

#[derive(Clone)]
pub struct JsCodeService {}

impl JsCodeService {
    pub fn new() -> Self {
        JsCodeService { }
    }

    pub async fn evaluate_from_task<T: Clone + 'static>(&self,
                                    task_id: uuid::Uuid,
                                    code: String,
                                    dbs: &DbServices,
                                    tr: &Transaction<'_>,
                                    engine: Arc<Mutex<Engine>>
    ) -> Result<T, ErrorDefinition> {
        let engine = Arc::new(Engine::new());
        let mut scope = Scope::new();
        match dbs.tasks.get_task_variables(task_id, None, tr).await {
            Ok(vars) => {
                for v in vars {
                    match v.data_type.id.as_str() {
                        "string" => scope.push(v.name.clone(), String::from(v.value.clone().as_str().unwrap())),
                        "number" => scope.push(v.name.clone(), v.value.clone().as_f64().unwrap()),
                        "date" =>   scope.push(v.name.clone(), NaiveDate::parse_from_str(v.value.clone().as_str().unwrap(), "%Y-%m-%d")),
                        "datetime" =>   scope.push(v.name.clone(), DateTime::parse_from_str(v.value.clone().as_str().unwrap(), "%Y-%m-%d %H:%M:%S %z")),
                        // "object" =>   scope.push(v.name.clone(), Value::from(v.value.clone().as_object().unwrap())),
                        "bool" => scope.push(v.name.clone(), v.value.clone().as_bool().unwrap()),

                        _ => scope.push(v.name.clone(), String::from(v.value.clone().as_str().unwrap()))
                    };
                }
            }
            Err(_) => {}
        }

        match engine.eval_with_scope::<T>(&mut scope, code.as_str()) {
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(ErrorDefinition::with_reason("Couldn't evaluate script".to_string(), json!({"error": format!("{}", err)})))
            }
        }
    }
}

