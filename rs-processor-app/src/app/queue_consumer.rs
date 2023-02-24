use std::str::{FromStr, Utf8Error};
use std::sync::Arc;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Channel, Consumer};
use log::{error, info};
use futures_lite::StreamExt;
use uuid::{Error, uuid, Uuid};
use rs_commons::adapters::db::client::PgClient;
use rs_commons::adapters::models::common_error::ErrorDefinition;
use rs_commons::adapters::models::worker::task_worker::{TaskWorker, WorkerWhat};
use rs_commons::adapters::queue_publisher::QueueConfig;
use rs_commons::db::services::{App, DbServices};
use crate::app::app_service::AppService;

pub struct QueueConsumer {
    conn: Arc<Connection>,
    config: Arc<QueueConfig>,
    channel: Arc<Channel>,
    consumer: Consumer,
}

impl QueueConsumer {
    pub async fn new(queue_config: QueueConfig) -> Result<Self, ErrorDefinition> {
        match Connection::connect(queue_config.amqp_path.clone().as_str(), ConnectionProperties::default()).await {
            Ok(conn) => {
                match conn.create_channel().await {
                    Ok(channel) => {
                        if let Err(err) = channel.queue_declare(
                            queue_config.queue_name.as_str(),
                            QueueDeclareOptions::default(),
                            FieldTable::default()
                        ).await {
                            return Err(ErrorDefinition::with_error("Couldn't declare queue", err))
                        }
                        match channel
                            .basic_consume(
                                "",
                                "worker",
                                BasicConsumeOptions::default(),
                                FieldTable::default()
                            ).await {
                            Ok(consumer) => {
                                Ok(QueueConsumer {
                                    conn: Arc::new(conn),
                                    channel: Arc::new(channel),
                                    config: Arc::new(queue_config),
                                    consumer
                                })
                            }
                            Err(err) => {
                                Err(ErrorDefinition::with_error("Couldn't create consumer", err))
                            }
                        }
                    }
                    Err(err) => {
                        Err(ErrorDefinition::with_error("Couldn't create channel", err))
                    }
                }
            }
            Err(err) => {
                Err(ErrorDefinition::with_error("Error creating amqp connection", err))
            }
        }
    }

    pub async fn run(&mut self, app: &AppService) -> Result<(), ErrorDefinition> {
        while let Some(delivery) = self.consumer.next().await {
            if let Ok(delivery) = delivery {
                match String::from_utf8(delivery.data) {
                    Ok(uuid) => {
                        let app = app.clone();
                        let db_client = app.db_client.clone();
                        let dbs = app.db_service.clone();
                        let db_service = app.db_service.clone();

                        match Uuid::from_str(uuid.as_str()) {
                            Ok(uuid) => {
                                match dbs.worker.get_worker(uuid, &db_client).await {
                                    Ok(w) => {
                                        match w.what {
                                            WorkerWhat::Process => {
                                                let _ = db_service.worker.process(w, &db_client, &dbs, &app.app).await;
                                            }
                                            WorkerWhat::RouteAfter => {
                                                let _ = db_service.worker.route_after(w, &db_client, &dbs, &app.app).await;
                                            }
                                        }
                                        let _ = self.channel
                                            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                                            .await;
                                    }
                                    Err(_) => {}
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        }

        Ok(())
    }
}