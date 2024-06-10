use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct DeleteEventSubCommand {
    /// Event id to delete. Must be in hex format.
    #[arg(short, long)]
    event_id: String,
    /// Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub async fn delete(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &DeleteEventSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    let event_id_to_delete = EventId::from_hex(sub_command_args.event_id.clone())?;

    let event_id = client.delete_event(event_id_to_delete).await?;
    if !sub_command_args.hex {
        println!("Deleted event with id: {}", event_id.to_bech32()?);
    } else {
        println!("Deleted event with id: {}", event_id.to_hex());
    }
    Ok(())
}
