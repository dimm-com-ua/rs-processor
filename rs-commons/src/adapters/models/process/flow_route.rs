use serde_json::Value;
use crate::adapters::models::process::flow_element::FlowElement;
use crate::db::models::process_db::FlowRouteDb;

#[derive(Debug)]
pub struct FlowRoute {
    pub model: FlowRouteDb,
    pub out_flow_element: Option<FlowElement>,
    pub in_flow_element: Option<FlowElement>
}