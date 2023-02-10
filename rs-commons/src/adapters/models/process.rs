use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;
use deadpool_postgres::Transaction;
use log::info;
use serde_json::{json, Value};
use crate::adapters::data_types::DataTypeTrait;
use crate::adapters::models::data_type::DataType;
use crate::adapters::models::handlers::HandlerType;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::flow_db::FlowElementArgumentDb;
use crate::db::services::{App, DbServices};

#[derive(Debug)]
pub struct ProcessFlow {
    pub id: uuid::Uuid,
    pub name: String,
    pub enabled: bool,
    pub version_id: i32
}

pub struct FlowElement {
    pub id: uuid::Uuid,
    pub el_type: String,
    pub handler_type: HandlerType,
    pub handler_value: Value,
    pub description: String
}

#[derive(Debug)]
pub enum ArgumentDirection { In, Out, Undefined }

#[derive(Debug)]
pub struct FlowElementArgument {
    pub id: uuid::Uuid,
    pub name: String,
    pub direction: ArgumentDirection,
    pub data_type: DataType,
    pub is_required: bool
}

impl FlowElementArgument {
    pub fn from_db(db_model: &FlowElementArgumentDb, dt: &DataTypeDb) -> Self {
        FlowElementArgument {
            id: db_model.id,
            name: db_model.arg_name.clone(),
            direction: match db_model.direction.to_lowercase().trim() { "in" => ArgumentDirection::In, "out" => ArgumentDirection::Out, _ => ArgumentDirection::Undefined },
            data_type: DataType::from_db(dt),
            is_required: db_model.is_required
        }
    }
}

#[derive(Debug)]
pub enum ProcessError { NotFound, GeneralError(String, Value) }

impl FlowElement {
    pub async fn get_all_arguments(&self, dbs: &DbServices, tr: &Transaction<'_>) -> Result<Vec<FlowElementArgument>, ProcessError> {
        match dbs.flow.get_flow_item_arguments(self.id, &tr).await {
            Ok(args) => {
                Ok(args)
            }
            Err(err) => {
                Err(ProcessError::GeneralError(format!("{:?}", err), json!({})))
            }
        }
    }

    pub async fn process(&self, args_to_process: Option<HashMap<String, Value>>, dbs: &DbServices, tr: &Transaction<'_>, app: &App) -> Result<(), ProcessError> {
        match self.get_all_arguments(dbs, &tr).await {
            Ok(args) => {
                let mut agrs_not_found: Vec<&FlowElementArgument> = args.iter().filter(|a| {
                    if a.is_required {
                        return if let Some(arg) = &args_to_process {
                            !arg.contains_key(a.name.as_str())
                        } else { true }
                    }
                    return false;
                }).collect();

                if agrs_not_found.len() > 0 {
                    return Err(ProcessError::GeneralError("Args that are not found".to_string(),
                                                          json!({"arguments": agrs_not_found.iter().map(|x| x.name.clone()).collect::<Vec<String>>()})))
                }

                for a in args {

                    match app.dt(a.data_type.id.clone()) {
                        None => { info!("Handler for {:?} not found", a.name.clone()) }
                        Some(dt) => {
                            info!("Found handler for: {:?}", a.name.clone());
                        }
                    }
                };
                Ok(())
            }
            Err(_) => {
                Err(ProcessError::NotFound)
            }
        }
    }
}