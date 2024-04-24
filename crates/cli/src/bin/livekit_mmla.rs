use clap::Parser;
use cli::auth::Auth;
use cli::livekit::LiveKit;

#[derive(Parser)]
enum LivekitMMLACLIOptions {
    #[clap(name = "auth", about = "Authenticate with Livekit MMLA server")]
    Auth(Auth),
    #[clap(name = "livekit", about = "Livekit MMLA server operations")]
    LiveKit(LiveKit),
}

fn main() {
    let opts = LivekitMMLACLIOptions::parse();
    match opts {
        LivekitMMLACLIOptions::Auth(auth) => {
            auth.execute();
        }
        LivekitMMLACLIOptions::LiveKit(livekit) => {
            livekit.execute();
        }
    }
}
