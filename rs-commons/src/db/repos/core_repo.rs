use deadpool_postgres::Transaction;
use serde_json::json;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::db::models::data_type_db::DataTypeDb;

#[derive(Clone)]
pub struct CoreRepo {}

impl CoreRepo {
    pub fn new() -> Self {
        CoreRepo{}
    }

    pub async fn get_data_type(&self, dt_name: String, tr: &Transaction<'_>) -> Result<DataTypeDb, ErrorDefinition> {
        let query = "select pdt from pc_data_type pdt where id=$1;";
        match tr.query(query, &[&dt_name]).await {
            Ok(rows) => {
                if rows.is_empty() {
                    return Err(ErrorDefinition::empty(format!("Data type {} not found", dt_name)))
                }
                match rows.get(0) {
                    None => { return Err(ErrorDefinition::empty(format!("Data type {} not found", dt_name))) }
                    Some(row) => {
                        let dt: DataTypeDb = row.get(0);
                        Ok(dt)
                    }
                }
            }
            Err(err) => {
                return Err(ErrorDefinition::with_reason("Error fetching data type".to_string(), json!({"error": format!("{:?}", err)})))
            }
        }
    }
}