use crate::utils::{delete_credentials, load_credentials, save_credentials, Credentials};
use clap::Parser;
use client::login_client::LoginClient;

#[derive(Debug, Parser, Clone)]
pub enum AuthSubCommand {
    #[clap(name = "login", about = "Login to Livekit MMLA server")]
    Login(Login),
    #[clap(name = "logout", about = "Logout from Livekit MMLA server")]
    Logout,
}

#[derive(Debug, Parser, Clone)]
pub struct Login {
    #[clap(
        short = 'b',
        long,
        help = "Base URL of the Livekit MMLA server",
        value_name = "BASE_URL"
    )]
    pub base_url: String,
    #[clap(
        short = 'u',
        long,
        help = "Username to login with",
        value_name = "USERNAME"
    )]
    pub username: String,
    #[clap(
        short = 'p',
        long,
        help = "Password to login with",
        value_name = "PASSWORD"
    )]
    pub password: String,
}

#[derive(Debug, Parser, Clone)]
pub struct Auth {
    // SubCommand
    #[clap(subcommand)]
    pub subcmd: Option<AuthSubCommand>,
}

impl Auth {
    pub fn execute(&self) -> () {
        match &self.subcmd {
            Some(subcmd) => match subcmd {
                AuthSubCommand::Login(login) => {
                    let client = LoginClient::new(&login.base_url);
                    let response = client.login(&login.username, &login.password);
                    match response {
                        Ok(token_response) => {
                            let creds = Credentials {
                                base_url: login.base_url.clone(),
                                token: token_response.token.clone(),
                                username: login.username.clone(),
                            };
                            let save_result = save_credentials(&creds);
                            match save_result {
                                Ok(_) => {
                                    println!("Login successful \n {:?}", creds);
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                }
                AuthSubCommand::Logout => {
                    let creds_result = load_credentials();
                    creds_result
                        .map(|creds| {
                            let client = LoginClient::new(&creds.base_url);
                            let response = client.logout(&creds.token);
                            match response {
                                Ok(_) => {
                                    delete_credentials()
                                        .map(|_| {
                                            println!("Logout successful");
                                        })
                                        .unwrap_or_else(|e| {
                                            println!("Error: {}. Error Deleting the files", e);
                                        });
                                }
                                Err(e) => {
                                    println!("Error: {:?}. Error from server", e);
                                }
                            }
                        })
                        .unwrap_or_else(|e| {
                            println!("Error: {}. Please login again", e);
                        });
                }
            },
            None => {
                println!("No subcommand provided");
            }
        }
    }
}
