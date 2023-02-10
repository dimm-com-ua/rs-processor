use actix_web::{get, web, HttpResponse, Responder};
use crate::api::tasks_api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1")
        .service(health_check)
        .service(web::scope("/task").configure(tasks_api::config)));
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
