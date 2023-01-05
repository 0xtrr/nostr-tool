use clap::{Args};

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;

#[derive(Args)]
pub struct RecommendRelaySubCommand {
    /// Relay URL to recommend
    #[arg(short, long)]
    url: String,
}

pub fn recommend_relay(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u16,
    sub_command_args: &RecommendRelaySubCommand,
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    let result = client
        .lock()
        .unwrap()
        .add_recommended_relay(&identity, sub_command_args.url.as_str(), difficulty_target);
    match result {
        Ok(_) => println!("Relay {} recommended", sub_command_args.url),
        Err(e) => eprintln!("{}", e)
    }
}