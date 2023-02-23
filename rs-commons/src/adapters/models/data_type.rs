use crate::db::models::data_type_db::DataTypeDb;

#[derive(Debug)]
pub struct DataType {
    pub id: String,
    pub name: String,
    pub handler: String,
}

impl DataType {
    pub fn from_db(db_model: &DataTypeDb) -> Self {
        DataType {
            id: db_model.id.clone(),
            name: db_model.name.clone(),
            handler: db_model.handler.clone(),
        }
    }
}
