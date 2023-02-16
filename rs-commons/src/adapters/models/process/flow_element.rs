use std::collections::HashMap;
use deadpool_postgres::Transaction;
use serde_json::{json, Value};
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::data_type::DataType;
use crate::adapters::models::handlers::HandlerType;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::flow_db::FlowElementArgumentDb;
use crate::db::services::{App, DbServices};

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

impl FlowElement {
    pub async fn get_all_arguments(&self, dbs: &DbServices, tr: &Transaction<'_>) -> Result<Vec<FlowElementArgument>, ErrorDefinition> {
        match dbs.flow.get_flow_item_arguments(self.id, &tr).await {
            Ok(args) => {
                Ok(args)
            }
            Err(err) => {
                Err(ErrorDefinition::from_db(&err))
            }
        }
    }

    pub async fn validate(&self, args_to_process: Option<HashMap<String, Value>>, dbs: &DbServices, tr: &Transaction<'_>, app: &App) -> Result<(), ErrorDefinition> {
        match self.get_all_arguments(dbs, &tr).await {
            Ok(args) => {
                let agrs_not_found: Vec<&FlowElementArgument> = args.iter().filter(|a| {
                    if a.is_required {
                        return if let Some(arg) = &args_to_process {
                            !arg.contains_key(a.name.as_str())
                        } else { true }
                    }
                    return false;
                }).collect();

                if agrs_not_found.len() > 0 {
                    return Err(ErrorDefinition::with_reason(
                        "Couldn't start the process. One or more required field(s) was not passed".to_string(),
                        json!({"fields": agrs_not_found.iter().map(|x| x.name.clone()).collect::<Vec<String>>()})
                    ))
                }
                let args_not_recognized: Vec<String> = args_to_process.as_ref().unwrap().clone().into_keys().filter(|x| {
                    args.iter().filter(|a| a.name == x.clone()).count() == 0
                }).collect();

                if args_not_recognized.len() > 0 {
                    return Err(ErrorDefinition::with_reason(
                        "Couldn't start the process. One or more fields couldn't be recognized".to_string(),
                        json!({"fields": args_not_recognized})
                    ))
                }

                let args_with_incorrect_handler: Vec<&FlowElementArgument> = args.iter().filter(|a| {
                    app.dt(a.data_type.id.clone()).is_none()
                }).collect();
                if args_with_incorrect_handler.len() > 0 {
                    return Err(ErrorDefinition::with_reason(
                        "Couldn't start the process. One or more fields have datatype with a handler that is not found".to_string(),
                        json!({"fields": args_with_incorrect_handler.iter().map(|x| x.name.clone()).collect::<Vec<String>>()})
                    ))
                }

                if let Some(args_to_process) = args_to_process {
                    let args_did_not_pass_validation: Vec<&FlowElementArgument> = args.iter().filter(|a| {
                        let val = &args_to_process.get(a.name.clone().as_str()).unwrap();
                        app.dt(a.data_type.id.clone()).unwrap().validate(val).is_err()
                    }).collect();
                    if args_did_not_pass_validation.len() > 0 {
                        return Err(ErrorDefinition::with_reason(
                            "Couldn't start the process. One or more fields did not pass validation".to_string(),
                            json!({"fields": args_did_not_pass_validation.iter().map(|x| x.name.clone()).collect::<Vec<String>>()})
                        ))
                    }
                }

                Ok(()) // Everything is Ok. We can pass variables to a process item
            }
            Err(_) => {
                Err(ErrorDefinition::empty("Not found".to_string()))
            }
        }
    }
}