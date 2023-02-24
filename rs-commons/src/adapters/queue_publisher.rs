use std::sync::Arc;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, options::*, types::FieldTable};
use log::info;
use crate::adapters::models::common_error::ErrorDefinition;

#[derive(Clone)]
pub struct QueuePublisher {
    channel: Arc<Channel>,
    config: Arc<QueueConfig>
}

#[derive(Clone)]
pub struct QueueConfig {
    pub amqp_path: String,
    pub queue_name: String
}

impl QueuePublisher {
    pub async fn new(queue_config: QueueConfig) -> Result<Self, ErrorDefinition> {
        match Connection::connect(queue_config.amqp_path.clone().as_str(), ConnectionProperties::default()).await {
            Ok(conn) => {
                match conn.create_channel().await {
                    Ok(channel) => {
                        match channel.queue_declare(
                            queue_config.queue_name.as_str(),
                            QueueDeclareOptions::default(),
                            FieldTable::default()
                        ).await {
                            Ok(_) => {
                                Ok(QueuePublisher {
                                    channel: Arc::new(channel),
                                    config: Arc::new(queue_config),
                                })
                            }
                            Err(err) => {
                                Err(ErrorDefinition::with_error("Couldn't declare queue", err))
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

    pub async fn create_worker(&self, uuid: uuid::Uuid) -> Result<(), ErrorDefinition> {let exchange_name = self.config.queue_name.clone();
        let headers = FieldTable::default();
        let properties = BasicProperties::default()
            .with_headers(headers)
            .with_content_type("text/plain".to_string().into());
        let payload = uuid.to_string();
        match self.channel
            .basic_publish(
                "",
                self.config.queue_name.as_str(),
                BasicPublishOptions::default(),
                payload.into_bytes().as_ref(),
                properties
            ).await {
            Ok(_) => { Ok(()) }
            Err(err) => {
                Err(ErrorDefinition::with_error("Couldn't publish message", err))
            }
        }
    }
}