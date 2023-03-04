use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct ResourceSubCommand {
    /// Resource url
    #[arg(short, long)]
    url: String,
    /// Resource mediatype (e.g. "audio/mpeg")
    #[arg(short, long)]
    mediatype: String,
    /// About the resource
    #[arg(short, long)]
    content: Option<String>,
    /// Event references
    #[arg(long, action = clap::ArgAction::Append)]
    etag: Vec<String>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

/// NOTE: NIP-30 is not finalized and may be subject to change!
pub fn create_resource(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &ResourceSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    // Set up tags
    let mut tags: Vec<Tag> = vec![];

    // Create and add resource tag
    let resource_tag = Tag::Generic(
        TagKind::Custom(String::from("resource")),
        vec![
            sub_command_args.url.clone(),
            sub_command_args.mediatype.clone(),
        ],
    );
    tags.push(resource_tag);

    // Add e-tag(s)
    for etag in sub_command_args.etag.iter() {
        let event_id = EventId::from_hex(etag)?;
        tags.push(Tag::Event(event_id, None, None));
    }

    let content = match sub_command_args.content.clone() {
        Some(c) => c,
        None => String::from(""),
    };

    // Create event
    let event_builder = EventBuilder::new(Kind::Custom(9), content, &tags);
    let event = event_builder.to_pow_event(&keys, difficulty_target)?;
    // Publish event
    let event_id = client.send_event(event)?;

    let id = if sub_command_args.hex {
        event_id.to_hex()
    } else {
        event_id.to_bech32()?
    };

    println!("Published resource with id: {}", id);
    Ok(())
}
