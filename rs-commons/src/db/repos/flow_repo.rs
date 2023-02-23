use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::flow_db::FlowElementArgumentDb;
use crate::db::repos::DbRepoError;
use deadpool_postgres::Transaction;
use serde_json::json;

#[derive(Clone)]
pub struct FlowRepo;

impl FlowRepo {
    pub fn new() -> Self {
        FlowRepo {}
    }

    pub async fn get_flow_item_arguments(
        &self,
        element_id: uuid::Uuid,
        tr: &Transaction<'_>,
    ) -> Result<Vec<(FlowElementArgumentDb, DataTypeDb)>, DbRepoError> {
        let query = "select ppfea, pdt from pc_process_flow_element_argument ppfea
                                left join pc_data_type pdt on pdt.id = ppfea.data_type
                            where flow_element_id=$1";
        match tr.query(query, &[&element_id]).await {
            Ok(rows) => {
                let args = rows
                    .iter()
                    .map(|r| {
                        let t: FlowElementArgumentDb = r.get(0);
                        let dt: DataTypeDb = r.get(1);
                        (t, dt)
                    })
                    .collect();
                Ok(args)
            }
            Err(err) => Err(DbRepoError::QueryError(format!(
                "Error fetching arguments list: {:?}",
                err
            ))),
        }
    }

    pub async fn save_flow_item_out_variables(
        &self,
        task_id: uuid::Uuid,
        element_id: uuid::Uuid,
        arg_names: Vec<String>,
        task_variables: Vec<TaskVariable>,
        tr: &Transaction<'_>,
    ) -> Result<Vec<(FlowElementArgumentDb, DataTypeDb)>, ErrorDefinition> {
        let mut args_with_error: Vec<(String, String)> = vec![];
        for arg in arg_names {
            let query = "delete from pc_task_variable where task_id=$1 and flow_element_id=$2;";
            match tr
                .query(query, &[&task_id.clone(), &element_id.clone()])
                .await
            {
                Ok(_) => {}
                Err(err) => args_with_error.push((arg.clone(), format!("{:?}", err))),
            }
            match task_variables.iter().find(|x| x.name == arg) {
                None => {}
                Some(var) => {
                    let query = "update pc_task_variable set value=$1 where task_id=$2 and name=$3 returning id;";
                    match tr
                        .query(query, &[&var.value.clone(), &task_id.clone(), &arg])
                        .await
                    {
                        Ok(rows) => {
                            if rows.is_empty() {
                                let query = "insert into pc_task_variable (task_id, name, data_type, value, flow_element_id) values ($1, $2, $3, $4, null);";
                                match tr
                                    .query(
                                        query,
                                        &[
                                            &task_id.clone(),
                                            &arg,
                                            &var.data_type.id.clone(),
                                            &var.value.clone(),
                                        ],
                                    )
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(err) => args_with_error
                                        .push((var.name.clone(), format!("{:?}", err))),
                                }
                            }
                        }
                        Err(err) => args_with_error.push((var.name.clone(), format!("{:?}", err))),
                    }
                }
            }
        }
        return if args_with_error.is_empty() {
            match self.get_flow_item_arguments(element_id.clone(), &tr).await {
                Ok(res) => Ok(res),
                Err(err) => Err(ErrorDefinition::with_reason(
                    "Db error".to_string(),
                    json!({ "error": format!("{:?}", err) }),
                )),
            }
        } else {
            Err(ErrorDefinition::with_reason(
                "Error saving results".to_string(),
                json!({ "fields_with_errors": args_with_error }),
            ))
        };
    }
}
