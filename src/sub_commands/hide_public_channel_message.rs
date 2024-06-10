use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct HidePublicChannelMessageSubCommand {
    /// Reason for hiding
    #[arg(short, long)]
    reason: Option<String>,
    /// Event to hide. Must be in hex format.
    #[arg(short, long)]
    event_id: String,
    /// Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub async fn hide_public_channel_message(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &HidePublicChannelMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    // Set up eventId
    let event_id_to_hide = EventId::from_hex(sub_command_args.event_id.clone())?;

    client.hide_channel_msg(event_id_to_hide, sub_command_args.reason.clone()).await?;
    println!("Channel message with id {event_id_to_hide} successfully hidden");

    Ok(())
}
