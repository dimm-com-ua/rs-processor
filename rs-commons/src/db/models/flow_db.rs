use postgres_types::{ToSql, FromSql};

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name="pc_process_flow_element_argument")]
pub struct FlowElementArgumentDb {
    pub id: uuid::Uuid,
    pub flow_element_id: uuid::Uuid,
    pub arg_name: String,
    pub direction: String,
    pub data_type: String,
    pub is_required: bool
}
