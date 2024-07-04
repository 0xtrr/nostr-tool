use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

#[derive(Args)]
pub struct SetChannelMetadataSubCommand {
    /// Channel ID
    #[arg(short, long)]
    channel_id: String,
    /// Channel name
    #[arg(short, long)]
    name: Option<String>,
    /// Channel about
    #[arg(short, long)]
    about: Option<String>,
    /// Channel picture
    #[arg(short, long)]
    picture: Option<String>,
    #[arg(short, long)]
    recommended_relay: Option<String>,
}

pub async fn set_channel_metadata(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SetChannelMetadataSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    // Process keypair and create a nostr client
    let keys = parse_private_key(private_key, true).await?;
    let client = create_client(&keys, relays.clone(), difficulty_target).await?;

    let channel_id: EventId = EventId::from_hex(sub_command_args.channel_id.clone())?;

    // Build metadata
    let mut metadata: Metadata = Metadata::new();

    if let Some(name) = sub_command_args.name.clone() {
        metadata = metadata.name(name);
    }

    if let Some(about) = sub_command_args.about.clone() {
        metadata = metadata.about(about);
    }

    if let Some(picture) = sub_command_args.picture.clone() {
        metadata = metadata.picture(Url::parse(picture.as_str()).unwrap());
    }

    let relay_url = sub_command_args
        .recommended_relay
        .clone()
        .map(|relay_string| Url::parse(relay_string.as_str()).unwrap());

    // Build and send event
    let event = EventBuilder::channel_metadata(channel_id, relay_url, &metadata).to_event(&keys)?;
    let event_id = client.send_event(event.clone()).await?;

    // Print results
    println!(
        "\nSet new metadata for channel {}!",
        sub_command_args.channel_id.as_str()
    );
    println!("\nEvent ID:");
    println!("Hex: {}", event_id.to_hex());
    println!("Bech32: {}", event_id.to_bech32()?);

    Ok(())
}
