use deadpool_postgres::Transaction;
use crate::adapters::models::handlers::HandlerType;
use crate::adapters::models::process::{FlowElement, ProcessFlow};
use crate::db::models::handlers_db::HandlerTypeDb;
use crate::db::models::process_db::{FlowElementDb, ProcessDefinitionDb, ProcessDefinitionFlowDb};
use crate::db::repos::DbRepoError;

#[derive(Clone)]
pub struct ProcessRepo;

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {}
    }

    pub async fn find_flow(&self, flow_code: &str, tr: &Transaction<'_>) -> Result<ProcessFlow, DbRepoError> {
        let query = "select ppd, ppdf from pc_process_definition_flow ppdf
                                left join pc_process_definition ppd on ppdf.process_id = ppd.id
                                where ppd.code=$1 order by ppdf.version_id desc limit 1;";
        match tr.query(query, &[&flow_code]).await {
            Ok(rows) => {
                return if rows.len() > 0 {
                    match rows.get(0) {
                        None => { Err(DbRepoError::NotFound) }
                        Some(row) => {
                            let ppd: ProcessDefinitionDb = row.get(0);
                            let ppdf: ProcessDefinitionFlowDb = row.get(1);
                            Ok(ProcessFlow {
                                id: ppdf.id,
                                name: ppd.name.clone(),
                                enabled: ppd.enabled,
                                version_id: ppdf.version_id
                            })
                        }
                    }
                } else {
                    Err(DbRepoError::NotFound)
                }
            }
            Err(err) => { Err(DbRepoError::QueryError(format!("Error fetching flow: {:?}", err))) }
        }
    }

    pub async fn find_starting_element(&self, flow_id: uuid::Uuid, tr: &Transaction<'_>) -> Result<FlowElement, DbRepoError> {
        let query = "select ppfe, pht from pc_process_flow_element ppfe
                                left join pc_handler_type pht on pht.id = ppfe.handler_type
                                where ppfe.process_flow=$1
                                and ppfe.handler_type=(select pht.id from pc_handler_type pht where pht.name='starting');";
        match tr.query(query, &[&flow_id]).await {
            Ok(rows) => {
                return if rows.len() > 0 {
                    if let Some(row) = rows.get(0) {
                        let ppfe : FlowElementDb = row.get(0);
                        let pht: HandlerTypeDb = row.get(1);
                        Ok(FlowElement {
                            id: ppfe.id,
                            el_type: ppfe.el_type,
                            handler_type: HandlerType::from_db(pht),
                            handler_value: ppfe.handler_value,
                            description: ppfe.description.unwrap_or("".to_string())
                        })
                    } else {
                        Err(DbRepoError::NotFound)
                    }
                } else {
                    Err(DbRepoError::NotFound)
                }
            },
            Err(err) => { Err(DbRepoError::QueryError(format!("Error fetching flow_item: {:?}", err))) }
        }
    }
}