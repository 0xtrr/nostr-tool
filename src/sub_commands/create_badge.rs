use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

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
}

pub async fn create_badge(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &CreateBadgeSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = parse_private_key(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    let image_size = match (
        sub_command_args.image_size_height,
        sub_command_args.image_size_width,
    ) {
        (Some(height), Some(width)) => Some(ImageDimensions { height, width }),
        _ => None,
    };

    let thumbnails = if let Some(thumb_url) = sub_command_args.thumb_url.clone() {
        let thumb_size = match (
            sub_command_args.thumb_size_height,
            sub_command_args.thumb_size_width,
        ) {
            (Some(width), Some(height)) => Some((width, height)),
            _ => None,
        };

        let url = UncheckedUrl::from(thumb_url);

        if let Some((width, height)) = thumb_size {
            vec![(url, Some(ImageDimensions { width, height }))]
        } else {
            vec![(url, None)]
        }
    } else {
        Vec::new()
    };

    let image_url: Option<UncheckedUrl> =
        sub_command_args.image_url.clone().map(UncheckedUrl::from);

    let event = EventBuilder::define_badge(
        sub_command_args.id.clone(),
        sub_command_args.name.clone(),
        sub_command_args.description.clone(),
        image_url,
        image_size,
        thumbnails,
    )
    .to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event).await?;
    println!("Published badge definition with id:");
    println!("Hex: {}", event_id.to_hex());
    println!("Bech32: {}", event_id.to_bech32()?);

    Ok(())
}
