use std::sync::Arc;

use client::ProjectClient;
use dotenvy::dotenv;
use livekit::{Room, RoomOptions};

use livekit_gstreamer::{GstMediaStream, LKParticipant, PublishOptions, VideoPublishOptions};
use shared::livekit_models::{TokenRequest, VideoGrantsWrapper};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    gstreamer::init().unwrap();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let base_url = std::env::var("SYNCFLOW_BASE_URL").expect("SYNCFLOW_BASE_URL must be set");
    let project_id = std::env::var("SYNCFLOW_PROJECT_ID").expect("SYNCFLOW_PROJECT_ID must be set");
    let syncflow_api_key = std::env::var("SYNCFLOW_API_KEY").expect("SYNCFLOW_API_KEY must be set");
    let syncflow_api_secret =
        std::env::var("SYNCFLOW_API_SECRET").expect("SYNCFLOW_API_SECRET must be set");

    let project_client =
        ProjectClient::new(base_url, project_id, syncflow_api_key, syncflow_api_secret);

    let session_request = Default::default();

    let session = project_client.create_session(&session_request).await?;
    log::info!("Created new session {:#?}", session);
    let token_request = TokenRequest {
        identity: "SyncFlowProjectClient".to_string(),
        name: Some("SyncFlow Project Client".to_string()),
        video_grants: VideoGrantsWrapper {
            room: session.name,
            room_create: false,
            room_join: true,
            can_publish: true,
            ..Default::default()
        },
    };

    let session_token = project_client
        .generate_session_token(&session.id, &token_request)
        .await?;

    let server_url = session_token.livekit_server_url.as_ref().unwrap();

    let (room, _) = Room::connect(server_url, &session_token.token, RoomOptions::default()).await?;

    let new_room = Arc::new(room);

    // Note: Make sure to replace the device_id with the correct device and the codecs and resolutions are supported by the device
    // This can be checked by running `v4l2-ctl --list-formats-ext -d /dev/video0` for example or using gst-device-monitor-1.0 Video/Source
    let mut stream = GstMediaStream::new(PublishOptions::Video(VideoPublishOptions {
        codec: "image/jpeg".to_string(),
        width: 1920,
        height: 1080,
        framerate: 30,
        device_id: "/dev/video2".to_string(),
    }));

    stream.start().await.unwrap();

    let mut participant = LKParticipant::new(new_room.clone());

    let track_sid = participant.publish_stream(&mut stream, None).await?;

    log::info!(
        "Connected to room: {} - {}",
        new_room.name(),
        String::from(new_room.sid().await)
    );
    log::info!("Published track with SID for one minute: {:?}", track_sid);
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    log::info!("Unpublishing track with SID: {}", track_sid);

    new_room.close().await?;

    stream.stop().await.unwrap();

    log::info!(
        "Stream stopped, disconnecting from room {}",
        new_room.name()
    );

    let session_info = project_client.stop_session(&session.id).await?;

    log::info!("Session stopped: {:#?}", session_info);
    Ok(())
}
