use amqprs::BasicProperties;
use shared::deployment_config::RabbitMQConfig;

use amqprs::channel::{
    BasicPublishArguments, Channel, ExchangeDeclareArguments, QueueBindArguments,
    QueueDeclareArguments,
};
use amqprs::connection::{Connection, OpenConnectionArguments};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionNotifierError {
    #[error("Failed to connect to RabbitMQ: {0}")]
    ConnectionError(#[from] amqprs::error::Error),

    #[error("Failed to declare queue: {0}")]
    QueueDeclareError(String),
}

pub struct SessionNotifier {
    rabbitmq_config: RabbitMQConfig,
    connection: Connection,
    channel: Channel,
}

impl SessionNotifier {
    pub async fn create(rabbitmq_config: RabbitMQConfig) -> Result<Self, SessionNotifierError> {
        let connection = Connection::open(&OpenConnectionArguments::new(
            &rabbitmq_config.host,
            rabbitmq_config.port,
            &rabbitmq_config.root_username,
            &rabbitmq_config.root_password,
        ))
        .await?;
        let channel = connection.open_channel(None).await?;

        Ok(Self {
            rabbitmq_config,
            connection,
            channel,
        })
    }

    pub async fn bind_routing_key(&self, routing_key: &str) -> Result<(), SessionNotifierError> {
        let exchange_name = &self.rabbitmq_config.exchange_name;
        let queue_name = &self.rabbitmq_config.queue_name;

        self.channel
            .queue_bind(QueueBindArguments::new(
                queue_name,
                exchange_name,
                routing_key,
            ))
            .await?;

        Ok(())
    }

    pub async fn publish(
        &self,
        routing_key: &str,
        message: Vec<u8>,
    ) -> Result<(), SessionNotifierError> {
        let exchange_name = &self.rabbitmq_config.exchange_name;

        let args = BasicPublishArguments::new(exchange_name, routing_key);

        self.channel
            .basic_publish(BasicProperties::default(), message, args)
            .await?;

        Ok(())
    }

    pub async fn initialize(&self) -> Result<String, SessionNotifierError> {
        let exchange_name = &self.rabbitmq_config.exchange_name;
        let queue_name = &self.rabbitmq_config.queue_name;

        let queue_declare_result = self
            .channel
            .queue_declare(QueueDeclareArguments::durable_client_named(queue_name))
            .await?;

        let queue_details = queue_declare_result.ok_or(SessionNotifierError::QueueDeclareError(
            format!("Failed to declare queue: {}", queue_name),
        ))?;

        self.channel
            .exchange_declare(ExchangeDeclareArguments::new(exchange_name, "topic"))
            .await?;

        Ok(queue_details.0)
    }
}

impl Clone for SessionNotifier {
    fn clone(&self) -> Self {
        Self {
            rabbitmq_config: self.rabbitmq_config.clone(),
            connection: self.connection.clone(),
            channel: self.channel.clone(),
        }
    }
}
