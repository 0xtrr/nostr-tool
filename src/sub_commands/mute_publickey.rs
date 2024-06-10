use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct MutePublickeySubCommand {
    /// Reason for muting
    #[arg(short, long)]
    reason: Option<String>,
    /// Public key to mute. Must be in hex format.
    #[arg(short, long)]
    public_key: String,
}

pub async fn mute_publickey(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &MutePublickeySubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, true).await?;
    let client = create_client(&keys, relays, difficulty_target).await?;

    // Set up pubkey to mute
    let pubkey_to_mute = key::PublicKey::from_str(sub_command_args.public_key.as_str())?;

    let event_id = client.mute_channel_user(
        pubkey_to_mute,
        sub_command_args.reason.clone(),
    ).await?;

    println!(
        "Public key {} muted in event {}",
        pubkey_to_mute.to_string(),
        event_id
    );

    Ok(())
}
