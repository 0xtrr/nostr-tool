use bitcoin::hashes::hex::FromHex;
use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys, parse_key};

#[derive(Args)]
pub struct SetChannelMetadataSubCommand {
    /// Channel ID
    #[arg(short, long)]
    channel_id: String,
    /// Recommended relay
    #[arg(short, long)]
    recommended_relay: Option<String>,
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

pub fn set_channel_metadata(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SetChannelMetadataSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    // Process keypair and create a nostr client
    let keys = handle_keys(private_key)?;
    let client = create_client(&keys, relays.clone(), difficulty_target)?;
  
    // Parse the channel id which can both be hex or bech32 encoded
    let hex_channel_id: String = parse_key(sub_command_args.channel_id.clone())?;

    // Build ChannelId object which is required in set_channel_metadata function
    let sha256 = bitcoin::hashes::sha256::Hash::from_hex(hex_channel_id.as_str())?;
    let channel_id = ChannelId::new(sha256, relays.clone());
    // Build relay URL
    let relay_url: Option<Url> = match &sub_command_args.recommended_relay {
        Some(url) => Some(Url::parse(url.as_str())?),
        None => None,
    };

    // Build updated metadata
    let mut metadata = Metadata::new()
        .name(sub_command_args.name.as_str());
    if let Some(about) = sub_command_args.about.clone() {
        metadata = metadata.about(about.as_str());
    }
    if let Some(picture) = sub_command_args.picture.clone() {
        metadata = metadata.picture(Url::parse(picture.as_str())?);
    }

    // Send event
    let event_id = client.set_channel_metadata(channel_id, relay_url, metadata)?;
    
    // Print results
    println!("\nSet new metadata for channel!");
    println!("Channel id: {}", sub_command_args.channel_id.as_str());
    println!("Name: {}", sub_command_args.name.as_str());
    
    if let Some(about) = sub_command_args.about.clone() {
        println!("About: {}", about.as_str());
    }

    if let Some(picture) = sub_command_args.picture.clone() {
        println!("Picture: {}", picture.as_str());
    }
    
    println!("Bech32 event id: {}", event_id.to_bech32()?);
    println!("Hex event id: {}", event_id.to_hex());

    Ok(())
}
