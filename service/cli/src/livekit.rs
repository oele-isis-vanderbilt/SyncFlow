use clap::Parser;

#[derive(Parser)]
pub struct LiveKit {}

impl LiveKit {
    pub fn execute(&self) -> () {
        println!("LiveKit::execute");
    }
}
