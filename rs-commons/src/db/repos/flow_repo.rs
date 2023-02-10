use deadpool_postgres::Transaction;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::flow_db::FlowElementArgumentDb;
use crate::db::repos::DbRepoError;

#[derive(Clone)]
pub struct FlowRepo;

impl FlowRepo {
    pub fn new() -> Self { FlowRepo {} }

    pub async fn get_flow_item_arguments(&self, element_id: uuid::Uuid, tr: &Transaction<'_>) -> Result<Vec<(FlowElementArgumentDb, DataTypeDb)>, DbRepoError> {
        let query = "select ppfea, pdt from pc_process_flow_element_argument ppfea
                                left join pc_data_type pdt on pdt.id = ppfea.data_type
                            where flow_element_id=$1";
        match tr.query(query, &[&element_id])
            .await {
            Ok(rows) => {
                let args = rows.iter().map(|r| {
                    let t: FlowElementArgumentDb = r.get(0);
                    let dt: DataTypeDb = r.get(1);
                    (t, dt) }
                ).collect();
                Ok(args)
            }
            Err(err) => { Err(DbRepoError::QueryError(format!("Error fetching arguments list: {:?}", err))) }
        }
    }
}