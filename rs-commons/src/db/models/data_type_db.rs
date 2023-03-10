use postgres_types::{FromSql, ToSql};

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "pc_data_type")]
pub struct DataTypeDb {
    pub id: String,
    pub name: String,
    pub handler: String,
}
