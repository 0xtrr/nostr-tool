use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct AwardBadgeSubCommand {
    /// Unique identifier for the badge (d tag in the event definition event)
    #[arg(short, long)]
    badge_id: String,
    /// Pubkey references
    #[arg(short, long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn award_badge(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &AwardBadgeSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex, true)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    let awarded_pubkeys = sub_command_args
        .ptag
        .iter()
        .map(|pubkey_string| {
            XOnlyPublicKey::from_str(pubkey_string).expect("Unable to parse public key")
        })
        .collect();

    let event = EventBuilder::award_badge(
        sub_command_args.badge_id.clone(),
        keys.public_key(),
        awarded_pubkeys,
    )
    .to_pow_event(&keys, difficulty_target)?;

    println!("{:?}", event.as_json());

    // Publish event
    let event_id = client.send_event(event)?;
    if !sub_command_args.hex {
        println!(
            "Published badge award event with id: {}",
            event_id.to_bech32()?
        );
    } else {
        println!("Published badge award event with id: {}", event_id.to_hex());
    }

    Ok(())
}
