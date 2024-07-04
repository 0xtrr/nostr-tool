use clap::Args;
use nostr_sdk::nostr::nips::nip05;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

#[derive(Args)]
pub struct SetMetadataSubCommand {
    /// Set profile name
    #[arg(short, long)]
    name: Option<String>,
    /// Set your bio
    #[arg(short, long)]
    about: Option<String>,
    /// Set your profile image URL
    #[arg(short, long)]
    picture: Option<String>,
    /// Set your NIP-05
    #[arg(long)]
    nip05: Option<String>,
    /// Set your LUD-06 LNURL
    #[arg(long)]
    lud06: Option<String>,
    /// Set your LUD-16 LN address
    #[arg(long)]
    lud16: Option<String>,
    /// Arbitrary fields not in the protocol. Use this syntax: "key:value"
    #[arg(short, long)]
    extra_field: Vec<String>,
    /// Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub async fn set_metadata(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SetMetadataSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = parse_private_key(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    let mut metadata = Metadata::new();

    // Name
    if let Some(name) = &sub_command_args.name {
        metadata = metadata.name(name);
    }

    // About
    if let Some(about) = &sub_command_args.about {
        metadata = metadata.about(about);
    }

    // Picture URL
    if let Some(picture_url) = &sub_command_args.picture {
        let url = Url::parse(picture_url)?;
        metadata = metadata.picture(url);
    };

    // NIP-05 identifier
    if let Some(nip05_identifier) = &sub_command_args.nip05 {
        // Check if the nip05 is valid
        nip05::verify(&keys.public_key(), nip05_identifier.as_str(), None).await?;
        metadata = metadata.nip05(nip05_identifier);
    }

    // LUD-06 string
    if let Some(lud06) = &sub_command_args.lud06 {
        metadata = metadata.lud06(lud06);
    }

    // LUD-16 string
    if let Some(lud16) = &sub_command_args.lud16 {
        metadata = metadata.lud16(lud16);
    }

    for ef in sub_command_args.extra_field.iter() {
        let sef: Vec<&str> = ef.split(':').collect();
        if sef.len() == 2 {
            metadata = metadata.custom_field(sef[0], sef[1])
        }
    }

    let event_id = client.set_metadata(&metadata).await?;
    println!("New metadata event: {}", event_id.to_bech32()?);

    Ok(())
}
