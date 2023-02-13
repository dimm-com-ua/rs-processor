use derive_more::{Display};
use actix_web::{HttpResponse, ResponseError};
use rs_commons::adapters::models::common_error::ErrorDefinition;

#[derive(Display, Debug)]
pub enum ProcessorError {
    NotFound,
    DbError(ErrorDefinition),
    InternalError(ErrorDefinition),
    ExternalError(ErrorDefinition),
    BadData(ErrorDefinition)
}

impl ResponseError for ProcessorError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ProcessorError::NotFound => { HttpResponse::NotFound().finish() },
            ProcessorError::InternalError(err) => { HttpResponse::InternalServerError().json(err.as_json()) },
            ProcessorError::DbError(err) => { HttpResponse::InternalServerError().json(err.as_json()) }
            _ => {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
