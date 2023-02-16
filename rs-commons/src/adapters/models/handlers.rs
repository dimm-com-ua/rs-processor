use crate::db::models::handlers_db::HandlerTypeDb;

pub struct HandlerType {
    pub name: String
}

impl HandlerType {
    pub fn from_db(db_model: HandlerTypeDb) -> Self {
        HandlerType {
            name: db_model.name.clone()
        }
    }
}