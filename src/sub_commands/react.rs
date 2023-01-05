use clap::{Args};

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;

#[derive(Args)]
pub struct ReactionSubCommand {
    /// Event id to react to
    #[arg(short, long)]
    event_id: String,
    /// Author pubkey of the event you are reacting to
    #[arg(short, long)]
    author_pubkey: String,
    /// Reaction content. Set to '+' for like or '-' for dislike. Single emojis are also often used for reactions, such as in Damus Web.
    #[arg(short, long)]
    reaction: String,
}

pub fn react_to_event(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u16,
    sub_command_args: &ReactionSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    if sub_command_args.reaction.trim().is_empty() {
        panic!("Reaction does not contain any content")
    }
    let result = client
        .lock()
        .unwrap()
        .react_to(
            &identity,
            &sub_command_args.event_id,
            &sub_command_args.author_pubkey,
            &sub_command_args.reaction,
            difficulty_target,
        );
    match result {
        Ok(event) => println!(
            "Reacted to {} with {} in event {}",
            &sub_command_args.event_id, sub_command_args.reaction, event.id
        ),
        Err(e) => eprintln!("{}", e)
    }
}