use std::fs;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct SendZapSubCommand {
    /// Bolt 11 invoice string
    #[arg(short, long)]
    bolt11: String,
    /// The path to a json document containing the zap request event json
    #[arg(short, long)]
    zap_request_json_path: String,
    /// Payment hash of the bolt11 invoice
    #[arg(short, long)]
    preimage: Option<String>,
    /// Pubkey references. Both hex and bech32 encoded keys are supported.
    #[arg(long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    /// Event references
    #[arg(long, action = clap::ArgAction::Append)]
    etag: Vec<String>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn send_zap_receipt(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SendZapSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    // Read in json from specified file
    let event_json: String = fs::read_to_string(sub_command_args.zap_request_json_path.clone())?;
    // Create Event from json
    let event = Event::from_json(event_json)?;

    let event: Event = EventBuilder::new_zap(
        sub_command_args.bolt11.clone(),
        sub_command_args.preimage.clone(),
        event,
    )
    .to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event)?;
    if !sub_command_args.hex {
        println!("Published zap receipt with id: {}", event_id.to_bech32()?);
    } else {
        println!("Published zap receipt with id: {}", event_id.to_hex());
    }

    Ok(())
}
