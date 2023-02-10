use crate::db::models::handlers_db::HandlerTypeDb;

pub struct HandlerType {
    name: String
}

impl HandlerType {
    pub fn from_db(db_model: HandlerTypeDb) -> Self {
        HandlerType {
            name: db_model.name.clone()
        }
    }
}