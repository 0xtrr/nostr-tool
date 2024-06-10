use std::time::Duration;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct DeleteProfileSubCommand {
    /// Delete just the events instead of the profile
    #[arg(long, default_value = "false")]
    events_only: bool,
    /// If events only are selected, allows specifying kinds
    #[arg(short, long, action = clap::ArgAction::Append)]
    kinds: Option<Vec<u64>>,
    /// Reason for deleting the events
    #[arg(short, long)]
    reason: Option<String>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
    /// Timeout in seconds
    #[arg(long)]
    timeout: Option<u64>,
}

pub async fn delete(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &DeleteProfileSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    let timeout = sub_command_args.timeout.map(Duration::from_secs);

    if sub_command_args.events_only {
        // go through all of the user events
        let authors: Vec<PublicKey> = vec![keys.public_key()];
        println!("checking author events...");

        // Convert kind number to Kind struct
        let kinds: Vec<Kind> = sub_command_args
            .kinds
            .clone()
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|x| x as u16)
            .map(Kind::from)
            .collect();

        let events: Vec<Event> =
            client.get_events_of(vec![Filter::new().authors(authors).kinds(kinds)], timeout).await?;

        let event_ids: Vec<EventIdOrCoordinate> = events
            .iter()
            .map(|event| EventIdOrCoordinate::from(event.id))
            .collect::<Vec<EventIdOrCoordinate>>();

        println!("Retrieved events to delete: {}", events.len());

        let delete_event: Event = EventBuilder::delete_with_reason(
            event_ids,
            sub_command_args.reason.clone().unwrap_or(String::new()),
        ).to_pow_event(&keys, difficulty_target).unwrap();

        let event_id = client.send_event(delete_event).await?;

        if !sub_command_args.hex {
            println!("All event deleted in event {}", event_id.to_bech32()?);
        } else {
            println!("All event deleted in event {}", event_id.to_hex());
        }
    } else {
        // Not a perfect delete but multiple clients trigger off of this metadata
        let metadata = Metadata::default()
            .name("Deleted")
            .display_name("Deleted")
            .about("Deleted")
            .custom_field("deleted", Value::Bool(true));

        let event_id = client.set_metadata(&metadata).await?;
        println!("Metadata updated ({})", event_id.to_bech32()?);
    }
    Ok(())
}
