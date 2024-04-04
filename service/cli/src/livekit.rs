use crate::utils::load_credentials;
use clap::{ArgAction, Parser};
use client::livekit_client::{LiveKitClient, TokenGeneratePermissions};

#[derive(Debug, Parser, Clone)]
pub enum LiveKitSubCommand {
    #[clap(name = "create-room", about = "Create a new Livekit Room")]
    CreateRoom(CreateRoom),
    #[clap(name = "delete-room", about = "Delete a Livekit Room")]
    DeleteRoom(DeleteRoom),
    #[clap(name = "list-rooms", about = "List all Livekit the LiveKit Rooms")]
    ListRooms,
    #[clap(name = "generate-token", about = "Generate a Livekit token")]
    GenerateToken(GenerateToken),
}

#[derive(Debug, Parser, Clone)]
pub struct CreateRoom {
    #[clap(
        short = 'n',
        long,
        help = "Name of the room to create",
        value_name = "ROOM_NAME"
    )]
    pub room_name: String,

    #[clap(
        short = 'p',
        long,
        help = "Maximum number of participants allowed in the room",
        value_name = "MAX_PARTICIPANTS",
        default_value = "10"
    )]
    pub max_participants: u32,

    #[clap(
        short = 'e',
        long,
        help = "Time in seconds after which the room will be deleted if empty",
        value_name = "EMPTY_TIMEOUT",
        default_value = "600"
    )]
    pub empty_timeout: u32,

    #[clap(
        short = 'm',
        long,
        help = "Metadata to associate with the room",
        value_name = "METADATA",
        default_value = "livekit-mmla"
    )]
    pub metadata: String,
}

#[derive(Debug, Parser, Clone)]
pub struct DeleteRoom {
    #[clap(
        short = 'n',
        long,
        help = "Name of the room to delete",
        value_name = "ROOM_NAME"
    )]
    pub room_name: String,
}

#[derive(Debug, Parser, Clone)]
pub struct GenerateToken {
    #[clap(
        short = 'i',
        long,
        help = "Identity of the user",
        value_name = "IDENTITY"
    )]
    pub identity: String,

    #[clap(short = 'n', long, help = "Room name", value_name = "ROOM_NAME")]
    pub room_name: String,

    #[clap(
        short = 'p',
        long,
        help = "Can publish streams/data",
        action=ArgAction::SetTrue,
    )]
    pub can_publish: bool,

    #[clap(
        short = 's',
        long,
        help = "Can subscribe to streams",
        action=ArgAction::SetFalse,
    )]
    pub can_subscribe: bool,

    #[clap(
        short = 'r',
        long,
        help = "Can record room",
        action=ArgAction::SetFalse,
    )]
    room_record: bool,

    #[clap(
        short = 'c',
        long,
        help = "Can create room",
        action=ArgAction::SetFalse,
    )]
    can_create_room: bool,
}

#[derive(Debug, Parser, Clone)]
pub struct LiveKit {
    #[clap(subcommand)]
    pub subcommand: Option<LiveKitSubCommand>,
}

impl LiveKit {
    pub fn execute(&self) {
        match load_credentials() {
            Ok(creds) => match &self.subcommand {
                Some(subcmd) => match subcmd {
                    LiveKitSubCommand::CreateRoom(create_args) => {
                        let client = LiveKitClient::new(&creds.base_url, &creds.token);
                        println!("Warning! Arguments except room name are ignored");
                        let response = client.create_room(create_args.room_name.as_str());
                        match response {
                            Ok(room) => {
                                println!("Room created: {:?}", room);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    }
                    LiveKitSubCommand::ListRooms => {
                        let client = LiveKitClient::new(&creds.base_url, &creds.token);
                        let response = client.list_rooms();
                        match response {
                            Ok(rooms) => {
                                println!("Rooms: {:?}", rooms);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    }
                    LiveKitSubCommand::DeleteRoom(options) => {
                        let client = LiveKitClient::new(&creds.base_url, &creds.token);
                        let response = client.delete_room(options.room_name.as_str());
                        match response {
                            Ok(r) => {
                                println!("Room deleted, {:?}", r);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    }
                    LiveKitSubCommand::GenerateToken(token_options) => {
                        let client = LiveKitClient::new(&creds.base_url, &creds.token);
                        let permissions =
                            if token_options.can_publish && token_options.can_subscribe {
                                TokenGeneratePermissions::PublishSubscribe
                            } else if token_options.can_publish {
                                TokenGeneratePermissions::Publish
                            } else if token_options.can_subscribe {
                                TokenGeneratePermissions::Subscribe
                            } else {
                                TokenGeneratePermissions::Publish
                            };

                        let response = client.generate_token(
                            token_options.identity.as_str(),
                            token_options.room_name.as_str(),
                            Some(permissions),
                            Some(token_options.room_record),
                            Some(token_options.can_create_room),
                        );

                        match response {
                            Ok(token) => {
                                println!("Token: {:?}", token);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    }
                },
                None => {
                    println!("No subcommand provided");
                }
            },
            Err(e) => {
                println!("Error loading credentials: {:?}. Please login again.", e);
            }
        }
    }
}
