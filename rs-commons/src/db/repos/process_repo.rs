use crate::adapters::models::handlers::HandlerType;
use crate::adapters::models::process::flow_route::FlowRoute;
use crate::adapters::models::process::{flow_element::FlowElement, process_flow::ProcessFlow};
use crate::db::models::handlers_db::HandlerTypeDb;
use crate::db::models::process_db::{
    FlowElementDb, FlowRouteDb, ProcessDefinitionDb, ProcessDefinitionFlowDb,
};
use crate::db::repos::DbRepoError;
use deadpool_postgres::Transaction;
use postgres_types::ToSql;

#[derive(Clone)]
pub struct ProcessRepo;

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {}
    }

    pub async fn find_flow(
        &self,
        flow_code: &str,
        tr: &Transaction<'_>,
    ) -> Result<ProcessFlow, DbRepoError> {
        let query = "select ppd, ppdf from pc_process_definition_flow ppdf
                                left join pc_process_definition ppd on ppdf.process_id = ppd.id
                                where ppd.code=$1 order by ppdf.version_id desc limit 1;";
        match tr.query(query, &[&flow_code]).await {
            Ok(rows) => {
                return if rows.len() > 0 {
                    match rows.get(0) {
                        None => Err(DbRepoError::NotFound),
                        Some(row) => {
                            let ppd: ProcessDefinitionDb = row.get(0);
                            let ppdf: ProcessDefinitionFlowDb = row.get(1);
                            Ok(ProcessFlow {
                                id: ppdf.id,
                                name: ppd.name.clone(),
                                enabled: ppd.enabled,
                                version_id: ppdf.version_id,
                            })
                        }
                    }
                } else {
                    Err(DbRepoError::NotFound)
                }
            }
            Err(err) => Err(DbRepoError::QueryError(format!(
                "Error fetching flow: {:?}",
                err
            ))),
        }
    }

    pub async fn find_starting_element(
        &self,
        flow_id: uuid::Uuid,
        tr: &Transaction<'_>,
    ) -> Result<FlowElement, DbRepoError> {
        let query = "select ppfe, pht from pc_process_flow_element ppfe
                                left join pc_handler_type pht on pht.id = ppfe.handler_type
                                where ppfe.process_flow=$1
                                and ppfe.handler_type=(select pht.id from pc_handler_type pht where pht.name='starting');";
        self.query_flow_element(&query, &[&flow_id], tr).await
    }

    pub async fn get_flow_element(
        &self,
        flow_element_id: uuid::Uuid,
        tr: &Transaction<'_>,
    ) -> Result<FlowElement, DbRepoError> {
        let query = "select ppfe, pht from pc_process_flow_element ppfe
                            left join pc_handler_type pht on pht.id = ppfe.handler_type
                            where ppfe.id=$1";
        self.query_flow_element(&query, &[&flow_element_id], tr)
            .await
    }

    pub async fn get_out_routes(
        &self,
        flow_element_id: uuid::Uuid,
        tr: &Transaction<'_>,
    ) -> Result<Vec<FlowRoute>, DbRepoError> {
        let query = "select ppfr, ppfo, pht from pc_process_flow_route ppfr
                              left join pc_process_flow_element ppfo on ppfo.id = ppfr.to_element
                                          left join pc_handler_type pht on pht.id = ppfo.handler_type
                            where ppfr.from_element = $1
                            order by ppfr.priority;";
        self.query_flow_routes(&query, &[&flow_element_id], tr)
            .await
    }

    async fn query_flow_routes(
        &self,
        query: &str,
        args: &[&(dyn ToSql + Sync)],
        tr: &Transaction<'_>,
    ) -> Result<Vec<FlowRoute>, DbRepoError> {
        match tr.query(query, args).await {
            Ok(rows) => {
                let ret: Vec<FlowRoute> = rows
                    .iter()
                    .map(|r| {
                        let ppfr: FlowRouteDb = r.get(0);
                        let ppfo: FlowElementDb = r.get(1);
                        let pppht: HandlerTypeDb = r.get(2);
                        FlowRoute {
                            model: ppfr,
                            out_flow_element: None,
                            in_flow_element: Some(FlowElement {
                                id: ppfo.id.clone(),
                                el_type: ppfo.el_type.clone(),
                                handler_type: HandlerType::from_db(pppht),
                                handler_value: ppfo.handler_value.clone(),
                                description: ppfo.description.unwrap_or("".to_string()),
                            }),
                        }
                    })
                    .collect();
                Ok(ret)
            }
            Err(err) => Err(DbRepoError::QueryError(format!(
                "Error fetching flow routes list: {:?}",
                err
            ))),
        }
    }

    async fn query_flow_element(
        &self,
        query: &str,
        args: &[&(dyn ToSql + Sync)],
        tr: &Transaction<'_>,
    ) -> Result<FlowElement, DbRepoError> {
        match tr.query(query, args).await {
            Ok(rows) => {
                return if rows.len() > 0 {
                    if let Some(row) = rows.get(0) {
                        let ppfe: FlowElementDb = row.get(0);
                        let pht: HandlerTypeDb = row.get(1);
                        Ok(FlowElement {
                            id: ppfe.id,
                            el_type: ppfe.el_type,
                            handler_type: HandlerType::from_db(pht),
                            handler_value: ppfe.handler_value,
                            description: ppfe.description.unwrap_or("".to_string()),
                        })
                    } else {
                        Err(DbRepoError::NotFound)
                    }
                } else {
                    Err(DbRepoError::NotFound)
                }
            }
            Err(err) => Err(DbRepoError::QueryError(format!(
                "Error fetching flow_item: {:?}",
                err
            ))),
        }
    }
}
