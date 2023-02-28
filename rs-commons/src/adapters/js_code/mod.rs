use std::ops::Deref;
use std::sync::Arc;

use deadpool_postgres::Transaction;
use log::info;
use rhai::{Dynamic, Engine, Scope};
use serde_json::json;

use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::{ArgumentDirection, FlowElementArgument};
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::db::services::DbServices;

#[derive(Clone)]
pub struct JsCodeService {}

pub mod vars;

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
                    let val = v.to_engine_value();
                    scope.push(v.name.clone(), val);
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
        code: &str,
        task_worker: &TaskWorker,
        dbs: Arc<&DbServices>,
        tr: Arc<&Transaction<'_>>,
    ) -> Result<Vec<TaskVariable>, ErrorDefinition> {
        let vars = match dbs
            .tasks
            .get_task_variables(
                task_worker.task_id.clone(),
                Some(task_worker.element_id.clone()),
                tr.deref(),
            )
            .await
        {
            Ok(vars_found) => vars_found,
            Err(_) => {
                vec![]
            }
        };

        let args = match dbs
            .flow
            .get_flow_item_arguments(task_worker.element_id, tr.deref())
            .await
        {
            Ok(args) => args,
            Err(_) => vec![],
        };
        let out_args: Vec<&FlowElementArgument> = args
            .iter()
            .filter(|a| a.direction == ArgumentDirection::Out)
            .collect();

        let mut scope = Scope::new();
        let engine = Engine::new();
        for v in vars {
            scope.push(v.name.clone(), v.to_engine_value());
        }

        // let code = format!(
        //     "   let val = value;
        //     let result = \"\";
        //     if(val>1000) {{
        //         result = \"accepted\";
        //     }} else {{
        //         result = \"rejected\";
        //     }};
        //     let decision = result;
        //     true
        // "
        // );
        let result = engine.eval_with_scope::<bool>(&mut scope, code);
        println!("{}", code);
        match result {
            Ok(_) => {
                let out_vars = out_args
                    .iter()
                    .filter_map(|x| match scope.get_value::<Dynamic>(x.name.as_str()) {
                        Some(val) => Some(TaskVariable::from_dynamic(x.name.clone(), val.clone())),
                        None => None,
                    })
                    .collect();
                Ok(out_vars)
            }
            Err(err) => {
                Err(ErrorDefinition::with_error("Couldn't run script", err))
            }
        }
    }
}
