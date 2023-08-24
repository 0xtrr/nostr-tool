use std::ops::Add;
use std::str::FromStr;
use std::time::Duration;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys, parse_key};

#[derive(Args)]
pub struct UserStatusSubCommand {
    /// Text note content
    #[arg(short, long)]
    content: String,
    /// Status type tag
    #[arg(short, long)]
    status_type: Option<String>,
    /// Pubkey references. Both hex and bech32 encoded keys are supported.
    #[arg(short, long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    /// Event references
    #[arg(short, long, action = clap::ArgAction::Append)]
    etag: Vec<String>,
    /// Reference tag
    #[arg(short, long, action = clap::ArgAction::Append)]
    rtag: Vec<String>,
    /// Seconds till expiration (NIP-40)
    #[arg(long)]
    expiration: Option<u64>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn set_user_status(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &UserStatusSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex, true)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    // Set up tags
    let mut tags: Vec<Tag> = vec![];

    // Add identifier tag
    if let Some(status) = &sub_command_args.status_type {
        let status = Tag::Identifier(status.to_string());
        tags.push(status);
    }

    // Add expiration tag
    if let Some(expiration) = sub_command_args.expiration {
        let timestamp = Timestamp::now().add(Duration::from_secs(expiration));
        tags.push(Tag::Expiration(timestamp));
    }

    // Add p-tags
    for ptag in sub_command_args.ptag.iter() {
        // Parse pubkey to ensure we're sending hex keys
        let pubkey_hex = parse_key(ptag.clone())?;
        let pubkey = XOnlyPublicKey::from_str(&pubkey_hex)?;
        tags.push(Tag::PubKey(pubkey, None));
    }

    // Add e-tag
    for etag in sub_command_args.etag.iter() {
        let event_id = EventId::from_hex(etag)?;
        tags.push(Tag::Event(event_id, None, None));
    }

    // Add r-tags
    for rtag in sub_command_args.rtag.iter() {
        let reference_tag = Tag::Reference(rtag.to_string());
        tags.push(reference_tag);
    }

    // Publish event
    let event = EventBuilder::new(Kind::Custom(30315), sub_command_args.content.clone(), &tags)
        .to_pow_event(&keys, difficulty_target)?;
    let event_id = client.send_event(event)?;
    if !sub_command_args.hex {
        println!("Published user status with id: {}", event_id.to_bech32()?);
    } else {
        println!("Published user status with id: {}", event_id.to_hex());
    }

    Ok(())
}
