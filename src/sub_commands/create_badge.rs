use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct CreateBadgeSubCommand {
    /// An unique name for the badge
    #[arg(short, long)]
    unique_name: String,
    /// A name tag with a short name for the badge
    #[arg(short, long)]
    name: Option<String>,
    /// An URL of a high-resolution image representing the badge
    #[arg(short, long)]
    image_url: Option<String>,
    /// Size in pixels, e.g. 1024x1024
    #[arg(long)]
    image_size: Option<String>,
    /// An URL pointing to a thumbnail version of the image referenced in the image_url
    #[arg(short, long)]
    thumb_url: Option<String>,
    /// Size in pixels, e.g. 256x256
    #[arg(long)]
    thumb_size: Option<String>,
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

    // Set up tags
    let mut tags: Vec<Tag> = vec![];

    // Add unique d tag
    tags.push(Tag::Generic(
        TagKind::D,
        vec![sub_command_args.unique_name.clone()],
    ));

    // Add name tag
    if let Some(name_tag) = sub_command_args.name.clone() {
        tags.push(Tag::Generic(
            TagKind::Custom(String::from("name")),
            vec![name_tag],
        ));
    }

    // Add an optional image URL and image size
    if let Some(image_url) = sub_command_args.image_url.clone() {
        if let Some(image_size) = sub_command_args.image_size.clone() {
            tags.push(Tag::Generic(TagKind::Image, vec![image_url, image_size]));
        } else {
            tags.push(Tag::Generic(TagKind::Image, vec![image_url]));
        }
    }

    // Add an optional thumbnail image URL and image size
    if let Some(thumbnail_url) = sub_command_args.thumb_url.clone() {
        if let Some(thumbnail_url_size) = sub_command_args.thumb_size.clone() {
            tags.push(Tag::Generic(
                TagKind::Image,
                vec![thumbnail_url, thumbnail_url_size],
            ));
        } else {
            tags.push(Tag::Generic(TagKind::Image, vec![thumbnail_url]));
        }
    }

    let event = EventBuilder::new(Kind::BadgeDefinition, "", &tags)
        .to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event)?;
    if !sub_command_args.hex {
        println!("Created badge with id: {}", event_id.to_bech32()?);
    } else {
        println!("Created badge with id: {}", event_id.to_hex());
    }

    Ok(())
}
