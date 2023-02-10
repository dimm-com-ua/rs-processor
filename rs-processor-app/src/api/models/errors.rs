use derive_more::{Display};
use actix_web::{HttpResponse, ResponseError};

#[derive(Display, Debug)]
pub enum ProcessorError {
    NotFound,
    DbError(String),
    InternalError(String),
    ExternalError(String),
    BadData(String)
}

impl ResponseError for ProcessorError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ProcessorError::NotFound => { HttpResponse::NotFound().finish() },
            ProcessorError::InternalError(err) => { HttpResponse::InternalServerError().body(err.clone()) },
            ProcessorError::DbError(err) => { HttpResponse::InternalServerError().body(err.clone()) }
            _ => {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
