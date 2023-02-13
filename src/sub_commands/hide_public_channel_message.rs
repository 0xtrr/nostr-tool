use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys, parse_key};

#[derive(Args)]
pub struct HidePublicChannelMessageSubCommand {
    /// Reason for hiding
    #[arg(short, long)]
    reason: Option<String>,
    /// Event to hide
    #[arg(short, long)]
    event_id: String,
}

pub fn hide_public_channel_message(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &HidePublicChannelMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    // Set up eventId
    let hex_event_id = parse_key(sub_command_args.event_id.clone())?;
    let event_id_to_hide = EventId::from_hex(hex_event_id)?;
    
    client.hide_channel_msg(event_id_to_hide.clone(), sub_command_args.reason.clone())?;
    println!("Channel message with id {} successfully hidden", event_id_to_hide);

    Ok(())
}
