use std::error::Error;

use client::ProjectClient;
use dotenvy::dotenv;
use shared::device_models::DeviceRegisterRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let base_url = std::env::var("SYNCFLOW_BASE_URL").expect("SYNCFLOW_BASE_URL must be set");
    let project_id = std::env::var("SYNCFLOW_PROJECT_ID").expect("SYNCFLOW_PROJECT_ID must be set");
    let syncflow_api_key = std::env::var("SYNCFLOW_API_KEY").expect("SYNCFLOW_API_KEY must be set");
    let syncflow_api_secret =
        std::env::var("SYNCFLOW_API_SECRET").expect("SYNCFLOW_API_SECRET must be set");

    let project_client =
        ProjectClient::new(base_url, project_id, syncflow_api_key, syncflow_api_secret);

    let new_device = DeviceRegisterRequest {
        name: "My Device".to_string(),
        group: "group-1".to_string(),
        comments: Some("Project Client Registered Device".to_string()),
    };

    let device_response = project_client.register_device(&new_device).await?;

    println!("Registered device: {:#?}", device_response);

    println!("Deleting device: {:#?}", device_response.id);

    let deleted_device = project_client.delete_device(&device_response.id).await?;

    println!("Deleted device: {:#?}", deleted_device);

    Ok(())
}
