use dotenv::dotenv;
use log;
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
    let response = client.get(&format!("{}", url)).send().await;
    match response {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}
