use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.0.0", display_name="livekit-mmla", about = "LiveKit MMLA CLI", long_about = None)]
#[command(propagate_version = true)]
struct LiveKitMMLA {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Users
}

#[derive(Subcommand)]
enum UserCmd {
    Add { username: String, email: String,  },
}



fn main() {
    let cli = LiveKitMMLA::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Users => {
            println!("Users subcommand selected.")
        }
    }
}