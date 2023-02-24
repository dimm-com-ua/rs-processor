use std::io::{Error, ErrorKind};
use std::sync::Arc;
use std::time::Duration;

use actix_web::middleware::Logger;
use actix_web::rt::time;
use actix_web::{web, App, HttpServer};
use log::info;
use rs_commons::adapters::db::config::DbConfiguration;
use rs_commons::adapters::models::common_error::ErrorDefinition;
use app::queue_consumer::QueueConsumer;
use rs_commons::adapters::queue_publisher::QueueConfig;

use rs_commons::config::config::Config;

use crate::api::api::config;
use crate::app::app_service::AppService;
use crate::app::worker_service::WorkerService;

mod api;
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = Arc::new(Config::make_from_env());
    let app_service = Arc::new(
        AppService::new(&app_config)
            .await
            .map_err(|err| {
                Error::new(
                    ErrorKind::Other,
                    format!("Couldn't start app_service! Error: {:?}", err),
                )
            })
            .unwrap(),
    );

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    {
        // app_service.lock().unwrap().prepare().await;
        app_service.prepare().await;
    }

    let app_arc_worker = app_service.clone();

    let app_config_clone = app_config.clone();
    actix_web::rt::spawn(async move {
        let _ = prepare_schedule(app_config_clone.get_queue_config().clone(), app_arc_worker)
            .await
            .expect("Failed to create schedule");
    });

    info!(
        "starting HTTP server at http://{}:{}",
        (&app_config).app_host,
        (&app_config).app_port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s"))
            .app_data(web::Data::new(app_service.clone()))
            .service(web::scope("/app").configure(config))
    })
    .bind((app_config.app_host.as_str(), (&app_config).app_port))?
    .run()
    .await
}

async fn prepare_schedule(config: QueueConfig, app: Arc<AppService>) -> Result<(), ErrorDefinition> {
    info!("Creating scheduler");
    let worker_service = WorkerService::new();
    match QueueConsumer::new(config).await {
        Ok(mut q_consumer) => {
            q_consumer.run(&app).await
        }
        Err(err) => {
            Err(err)
        }
    }
}
