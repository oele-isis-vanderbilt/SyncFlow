use client::http_client::ClientError;
use client::livekit_client::LiveKitClient;

const SYNCFLOW_URL: &str = "https://api.syncflow.live";
const LIVEKIT_URL: &str = "https://livekit.syncflow.live";

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let project = String::from("DemoProject");
    let syncflow_api_key = std::env::var("SYNCFLOW_API_KEY").expect("SYNCFLOW_API_KEY not set");
    let syncflow_api_secret =
        std::env::var("SYNCFLOW_API_SECRET").expect("SYNCFLOW_API_SECRET not set");

    let client = LiveKitClient::from_api_keys(
        SYNCFLOW_URL,
        &syncflow_api_key,
        &syncflow_api_secret,
        Some(project),
        None,
    )?;

    let room_name = "DemoRoom";

    let create_room_response = client.create_room(&room_name)?;

    println!("Room created: {:?}", create_room_response);

    let room_join_token = client.generate_token("device-1", room_name, None, None, None)?;

    println!("Room join token: {:?}", room_join_token);

    Ok(())
}
