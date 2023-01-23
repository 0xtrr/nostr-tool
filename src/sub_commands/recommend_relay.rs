use clap::Args;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct RecommendRelaySubCommand {
    /// Relay URL to recommend
    #[arg(short, long)]
    url: String,
}

pub fn recommend_relay(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &RecommendRelaySubCommand,
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key);
    let client = create_client(&keys, relays, difficulty_target);

    match client.add_recommended_relay(sub_command_args.url.clone()) {
        Ok(_) => println!("Relay {} recommended", sub_command_args.url),
        Err(e) => eprintln!("{e}"),
    }
}
