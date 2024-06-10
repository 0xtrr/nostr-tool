use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

#[derive(Args)]
pub struct CreatePublicChannelSubCommand {
    /// Channel name
    #[arg(short, long)]
    name: String,
    /// Channel about
    #[arg(short, long)]
    about: Option<String>,
    /// Channel picture
    #[arg(short, long)]
    picture: Option<String>,
}

pub async fn create_public_channel(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &CreatePublicChannelSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    // Process keypair and create a nostr client
    let keys = parse_private_key(private_key, true).await?;
    let client = create_client(&keys, relays.clone(), difficulty_target).await?;

    // Create metadata
    let mut metadata: Metadata = Metadata::new().name(sub_command_args.name.clone());

    if let Some(about) = sub_command_args.about.clone() {
        metadata = metadata.about(about);
    }

    if let Some(picture) = sub_command_args.picture.clone() {
        metadata = metadata.picture(Url::parse(picture.as_str()).unwrap());
    }

    // Send event
    let event: Event = EventBuilder::channel(&metadata).to_event(&keys).unwrap();
    let event_id = client.send_event(event).await?;

    // Print results
    println!("\nCreated new public channel!");
    println!("Channel ID:");
    println!("Hex: {}", event_id.to_hex());
    println!("Bech32: {}", event_id.to_bech32()?);

    Ok(())
}
