use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys, parse_key};

#[derive(Args)]
pub struct AwardBadgeSubCommand {
    /// Badge identifier
    #[arg(short, long)]
    badge_identifier: String,
    /// Pubkey references. Both hex and bech32 encoded keys are supported.
    #[arg(short, long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    /// Badge definition event id
    #[arg(short, long)]
    atag: String,
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

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays.clone(), difficulty_target)?;

    // Set up tags
    let mut tags: Vec<Tag> = vec![];

    // Add p-tags
    for ptag in sub_command_args.ptag.iter() {
        // Parse pubkey to ensure we're sending hex keys
        let pubkey_hex = parse_key(ptag.clone())?;
        let pubkey = XOnlyPublicKey::from_str(&pubkey_hex)?;
        tags.push(Tag::PubKey(pubkey, None));
    }

    // Add the a-tag
    tags.push(Tag::A {
        kind: Kind::BadgeDefinition,
        public_key: keys.public_key(),
        identifier: sub_command_args.badge_identifier.clone(),
        relay_url: UncheckedUrl::from_str(relays[0].as_str())?,
    });

    let event =
        EventBuilder::new(Kind::BadgeAward, "", &tags).to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event)?;

    if !sub_command_args.hex {
        println!("Badge award update event id: {}", event_id.to_bech32()?);
    } else {
        println!("Badge award update event id: {}", event_id.to_hex());
    }
    if !sub_command_args.hex {
        sub_command_args
            .ptag
            .iter()
            .for_each(|pubkey| println!("Awarded badge to {}", pubkey));
    }

    Ok(())
}
