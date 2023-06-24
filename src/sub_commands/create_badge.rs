use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct CreateBadgeSubCommand {
    /// Unique identifier for the badge
    #[arg(short, long)]
    id: String,
    ///
    #[arg(short, long)]
    name: Option<String>,
    ///
    #[arg(short, long)]
    description: Option<String>,
    ///
    #[arg(long)]
    image_url: Option<String>,
    ///
    #[arg(long)]
    image_size_width: Option<u64>,
    ///
    #[arg(long)]
    image_size_height: Option<u64>,
    ///
    #[arg(short, long)]
    thumb_url: Option<String>,
    ///
    #[arg(long)]
    thumb_size_width: Option<u64>,
    ///
    #[arg(long)]
    thumb_size_height: Option<u64>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn create_badge(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &CreateBadgeSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    let image_size = match (
        sub_command_args.image_size_height,
        sub_command_args.image_size_width,
    ) {
        (Some(height), Some(width)) => Some((height, width)),
        _ => None,
    };

    let thumb_size = match (
        sub_command_args.thumb_size_height,
        sub_command_args.thumb_size_width,
    ) {
        (Some(height), Some(width)) => Some((height, width)),
        _ => None,
    };

    let event = EventBuilder::define_badge(
        sub_command_args.id.clone(),
        sub_command_args.name.clone(),
        sub_command_args.description.clone(),
        sub_command_args.image_url.clone(),
        image_size,
        sub_command_args.thumb_url.clone(),
        thumb_size,
    )
    .to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event)?;
    if !sub_command_args.hex {
        println!(
            "Published badge definition with id: {}",
            event_id.to_bech32()?
        );
    } else {
        println!("Published badge definition with id: {}", event_id.to_hex());
    }

    Ok(())
}
