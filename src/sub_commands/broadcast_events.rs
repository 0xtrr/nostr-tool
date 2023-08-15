use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct BroadcastEventsSubCommand {
    /// Input file path, should contain an array of JSON events
    #[arg(short, long)]
    file_path: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn broadcast_events(
    relays: Vec<String>,
    sub_command_args: &BroadcastEventsSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(None, sub_command_args.hex, false)?;
    let client = create_client(&keys, relays.clone(), 0)?;

    let file = std::fs::File::open(&sub_command_args.file_path)?;

    let events: Vec<Event> = serde_json::from_reader(file)?;

    for event in events.clone() {
        client.send_event(event)?;
    }

    println!("Published {} events to {:?}", events.len(), relays);

    Ok(())
}
