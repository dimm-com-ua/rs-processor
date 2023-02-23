use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::db::services::DbServices;
use chrono::{DateTime, NaiveDate};
use deadpool_postgres::Transaction;
use rhai::{Engine, Scope};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct JsCodeService {}

impl JsCodeService {
    pub fn new() -> Self {
        JsCodeService {}
    }

    pub async fn evaluate_from_task<T: Clone + 'static>(
        &self,
        task_id: uuid::Uuid,
        code: String,
        dbs: &DbServices,
        tr: &Transaction<'_>,
    ) -> Result<T, ErrorDefinition> {
        let engine = Arc::new(Engine::new());
        let mut scope = Scope::new();
        match dbs.tasks.get_task_variables(task_id, None, tr).await {
            Ok(vars) => {
                for v in vars {
                    match v.data_type.id.as_str() {
                        "string" => scope.push(
                            v.name.clone(),
                            String::from(v.value.clone().as_str().unwrap()),
                        ),
                        "number" => scope.push(v.name.clone(), v.value.clone().as_f64().unwrap()),
                        "date" => scope.push(
                            v.name.clone(),
                            NaiveDate::parse_from_str(
                                v.value.clone().as_str().unwrap(),
                                "%Y-%m-%d",
                            ),
                        ),
                        "datetime" => scope.push(
                            v.name.clone(),
                            DateTime::parse_from_str(
                                v.value.clone().as_str().unwrap(),
                                "%Y-%m-%d %H:%M:%S %z",
                            ),
                        ),
                        // "object" =>   scope.push(v.name.clone(), Value::from(v.value.clone().as_object().unwrap())),
                        "bool" => scope.push(v.name.clone(), v.value.clone().as_bool().unwrap()),

                        _ => scope.push(
                            v.name.clone(),
                            String::from(v.value.clone().as_str().unwrap()),
                        ),
                    };
                }
            }
            Err(_) => {}
        }

        match engine.eval_with_scope::<T>(&mut scope, code.as_str()) {
            Ok(result) => Ok(result),
            Err(err) => Err(ErrorDefinition::with_reason(
                "Couldn't evaluate script".to_string(),
                json!({ "error": format!("{}", err) }),
            )),
        }
    }

    pub async fn evaluate_from_flow_element(
        &self,
        _code: &str,
        _task_worker: &TaskWorker,
        _dbs: &DbServices,
        _tr: &Transaction<'_>,
    ) -> Result<bool, ErrorDefinition> {
        let _engine = Engine::new();
        Ok(true)
    }
}
