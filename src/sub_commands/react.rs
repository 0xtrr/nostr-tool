use std::process::exit;
use std::time::Duration;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

#[derive(Args)]
pub struct ReactionSubCommand {
    /// Event id to react to
    #[arg(short, long)]
    event_id: String,
    /// Author pubkey of the event you are reacting to. Must be hex format.
    #[arg(short, long)]
    author_pubkey: String,
    /// Reaction content. Set to '+' for like or '-' for dislike. Single emojis are also often used for reactions, such as in Damus Web.
    #[arg(short, long)]
    reaction: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub async fn react_to_event(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &ReactionSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = parse_private_key(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    if sub_command_args.reaction.trim().is_empty() {
        eprintln!("Reaction does not contain any content");
        exit(0)
    }

    let event_id = EventId::from_hex(&sub_command_args.event_id)?;
    let author_pubkey = PublicKey::from_hex(sub_command_args.author_pubkey.clone())?;

    let subscription = Filter::new().event(event_id).author(author_pubkey);

    let events = client
        .get_events_of_with_opts(
            vec![subscription],
            Some(Duration::from_secs(30)),
            FilterOptions::ExitOnEOSE,
        )
        .await?;

    if events.is_empty() {
        eprintln!("Unable to find note with the provided event id");
        exit(0);
    }

    let event_to_react_to = events.first().unwrap();

    let id = client
        .reaction(event_to_react_to, sub_command_args.reaction.clone())
        .await?;
    println!(
        "Reacted to {} with {} in event {}",
        event_id.to_bech32()?,
        sub_command_args.reaction,
        id.to_bech32()?
    );
    Ok(())
}
