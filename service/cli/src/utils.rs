use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub token: String,
    pub base_url: String,
    pub username: String,
}

pub fn load_credentials() -> Result<Credentials, String> {
    let home_dir = dirs::home_dir();

    match home_dir {
        Some(home) => {
            let cred_path = home.join(".livekit_mmla/credentials.json");
            if !cred_path.exists() {
                Err("Credentials file not found".to_string())
            } else {
                let cred_file = std::fs::read_to_string(cred_path);
                match cred_file {
                    Ok(cred) => {
                        let creds: Credentials = serde_json::from_str(&cred).unwrap();
                        Ok(creds)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        None => Err("Could not find home directory".to_string()),
    }
}

pub fn delete_credentials() -> Result<(), String> {
    let home_dir = dirs::home_dir();

    match home_dir {
        Some(home) => {
            let cred_path = home.join(".livekit_mmla/credentials.json");
            if !cred_path.exists() {
                Err("Credentials file not found".to_string())
            } else {
                let res = std::fs::remove_file(cred_path);
                match res {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        None => Err("Could not find home directory".to_string()),
    }
}

pub fn save_credentials(creds: &Credentials) -> Result<(), String> {
    let home_dir = dirs::home_dir();

    match home_dir {
        Some(home) => {
            let dir_path = home.join(".livekit_mmla");

            // create the directory if it doesn't exist
            if !dir_path.exists() {
                let res = std::fs::create_dir(&dir_path);
                match res {
                    Ok(_) => (),
                    Err(e) => return Err(e.to_string()),
                }
            }

            let json_data = serde_json::to_string_pretty(creds).unwrap();

            // write the json data to a file
            let file_path = dir_path.join("credentials.json");
            let res = std::fs::write(file_path, json_data.clone());
            match res {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
        None => Err("Could not find home directory".to_string()),
    }
}
