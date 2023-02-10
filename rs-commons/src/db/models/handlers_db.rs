use postgres_types::{ToSql, FromSql};

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name="pc_handler_type")]
pub struct HandlerTypeDb {
    pub id: i32,
    pub name: String
}