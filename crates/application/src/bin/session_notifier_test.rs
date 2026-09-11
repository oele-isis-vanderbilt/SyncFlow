use std::error::Error;

use application::rmq::session_notifier::SessionNotifier;
use shared::deployment_config::DeploymentConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = DeploymentConfig::load();
    let queue_name = config.rabbitmq_config.queue_name.clone();
    let session_notifier = SessionNotifier::create(config.rabbitmq_config).await?;

    session_notifier.initialize().await?;
    println!("Queue name: {}", queue_name);

    for i in 0..10 {
        let message = format!("Message {}", i);
        session_notifier
            .publish("project-1.group-2", message.as_bytes().to_vec())
            .await?;
        println!("Sent message: {}", message);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
