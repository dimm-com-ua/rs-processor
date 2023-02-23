use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use rs_commons::adapters::models::common_error::ErrorDefinition;

#[derive(Display, Debug)]
pub enum ProcessorError {
    InternalError(ErrorDefinition),
    _ExternalError(ErrorDefinition),
    _BadData(ErrorDefinition),
}

impl ResponseError for ProcessorError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ProcessorError::InternalError(err) => {
                HttpResponse::InternalServerError().json(err.as_json())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
