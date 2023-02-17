use std::collections::HashMap;
use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::{Error, Row};
use deadpool_postgres::Transaction;
use log::info;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::data_type::DataType;
use crate::adapters::models::process::flow_element::FlowElementArgument;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::task_db::TaskVariableDb;

#[derive(Clone)]
pub struct TasksDbRepo;

impl TasksDbRepo {
    pub async fn create_task(&self, flow_uuid: Uuid, current_flow_id: Uuid, created_at: DateTime<Utc>, args: Option<HashMap<String, Value>>, element_args: Vec<FlowElementArgument>, tr: &Transaction<'_>) -> Result<Uuid, ErrorDefinition> {
        let query = "insert into pc_task (process_flow, created_at, current_flow_item) values ($1, $2, $3) returning id;";
        match tr.query(query, &[&flow_uuid, &created_at, &current_flow_id])
            .await {
            Ok(rows) => {
                if let Some(row) = rows.last() {
                    let uuid: Uuid = row.get(0);
                    if args.is_some() {
                        for (arg, value) in args.unwrap().iter() {
                            let data_type = if let Some(elm_arg) = element_args.iter().find(|x| x.name.clone() == arg.clone()) {
                                elm_arg.data_type.id.clone()
                            }  else {
                                "string".to_string()
                            };
                            let query = "insert into pc_task_variable (task_id, name, data_type, value, flow_element_id) values ($1, $2, $3, $4, $5);";
                            if let Err(err) = tr.query(query, &[&uuid, &arg, &data_type, value, &current_flow_id]).await {
                                return Err(ErrorDefinition::with_reason("Error inserting flow variable".to_string(), json!({"error": format!("{:?}", err)})))
                            }
                        }
                    }
                    Ok(uuid)
                } else {
                    Err(ErrorDefinition::empty("Couldn't get uuid of the created task".to_string()))
                }
            }
            Err(err) => {
                Err(ErrorDefinition::with_reason("Couldn't create task".to_string(), json!({"error": format!("{:?}", err)})))
            }
        }
    }

    pub async fn create_worker(&self, task_id: Uuid, element_id: Uuid, created_at: DateTime<Utc>, run_after: Option<DateTime<Utc>>, tr: &Transaction<'_>) -> Result<Uuid, ErrorDefinition> {
        let query = "insert into pc_task_worker (task_id, element_id, created_at, run_after) values ($1, $2, $3, $4) returning id;";
        match tr.query(query, &[&task_id, &element_id, &created_at, &run_after]).await {
            Ok(rows) => {
                if let Some(row) = rows.last() {
                    let uuid: Uuid = row.get(0);
                    Ok(uuid)
                } else {
                    Err(ErrorDefinition::empty("Couldn't get uuid of created worker".to_string()))
                }
            }
            Err(err) => {
                Err(ErrorDefinition::with_reason("Couldn't create task worker".to_string(), json!({"error": format!("{:?}", err)})))
            }
        }
    }

    pub async fn get_task_variables(&self, task_id: Uuid, element_id: Option<Uuid>, tr: &Transaction<'_>) -> Result<Vec<TaskVariable>, ErrorDefinition> {
        let query = "select ptv, pdt from pc_task_variable ptv
                                left join pc_data_type pdt on pdt.id = ptv.data_type
                                where task_id=$1 and flow_element_id=$2;";
        info!("{:?} - {:?}", task_id, element_id);
        match tr.query(query, &[&task_id, &element_id]).await {
            Ok(rows) => {
                let vars: Vec<TaskVariable> = rows.iter().map(|x| {
                    let ptv: TaskVariableDb = x.get(0);
                    let pdt: DataTypeDb = x.get(1);
                    TaskVariable {
                        id: ptv.id,
                        name: ptv.name,
                        data_type: DataType::from_db(&pdt),
                        value: ptv.value,
                    }
                }).collect();
                Ok(vars)
            }
            Err(err) => {
                Err(ErrorDefinition::with_reason("Couldn't get task variables".to_string(), json!({"error": format!("{:?}", err)})))
            }
        }
    }
}

impl TasksDbRepo {
    pub fn new() -> Self { TasksDbRepo{} }
}