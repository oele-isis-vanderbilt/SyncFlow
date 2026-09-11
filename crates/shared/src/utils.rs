use dotenvy::dotenv;
use livekit_protocol::egress_info::Request;
use livekit_protocol::{EgressInfo, EgressStatus};
use log;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest;

pub fn load_env() {
    match dotenv() {
        Ok(_) => {}
        Err(e) => {
            log::error!(
                "Failed to load .env file: {}, assuming variables are set",
                e
            );
        }
    };
}

pub fn get_livekit_server_http_url() -> String {
    let url = std::env::var("LIVEKIT_SERVER_URL").expect("LIVEKIT_SERVER_URL must be set");

    if url.starts_with("ws") {
        url.replace("ws", "http")
    } else if url.starts_with("wss") {
        url.replace("wss", "https")
    } else {
        url
    }
}

pub async fn ping_livekit() -> bool {
    let url = get_livekit_server_http_url();
    let client = reqwest::Client::new();
    let response = client.get(url).send().await;
    match response {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

pub fn generate_random_session_name() -> String {
    let prefix = "session";
    let rng = thread_rng();
    let session_name1: String = rng
        .clone()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    let session_name2: String = rng
        .clone()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    format!("{}-{}-{}", prefix, session_name1, session_name2)
}

pub fn get_track_id_from_egress(egress: &EgressInfo) -> String {
    if let Some(request) = egress.request.clone() {
        match request {
            Request::RoomComposite(_) => "RoomComposite".to_string(),
            Request::Participant(_) => "Participant".to_string(),
            Request::Track(req) => req.track_id.clone(),
            Request::TrackComposite(req) => {
                format!(
                    "TrackComposite-{}-{}",
                    req.audio_track_id, req.video_track_id
                )
            }
            Request::Web(_) => "Web".to_string(),
        }
    } else {
        "Unknown".to_string()
    }
}

pub fn get_egress_destination(egress: &EgressInfo) -> Option<String> {
    if egress.status() == EgressStatus::EgressComplete {
        let all_destinations = egress
            .file_results
            .iter()
            .map(|dest| dest.filename.clone())
            .collect::<Vec<String>>();

        Some(all_destinations.join(","))
    } else {
        None
    }
}
