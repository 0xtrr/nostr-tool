use crate::utils::{create_client, handle_keys, parse_key};
use clap::Args;
use nostr_sdk::prelude::*;

#[derive(Args)]
pub struct MutePublickeySubCommand {
    /// Reason for muting
    #[arg(short, long)]
    reason: Option<String>,
    /// Public key to mute
    #[arg(short, long)]
    public_key: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn mute_publickey(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &MutePublickeySubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    // Set up pubkey to mute
    let hex_pubkey = parse_key(sub_command_args.public_key.clone())?;
    let pubkey_to_mute = Keys::from_pk_str(hex_pubkey.as_str())?;

    let event_id =
        client.mute_channel_user(pubkey_to_mute.public_key(), sub_command_args.reason.clone())?;
    println!(
        "Public key {} muted in event {}",
        pubkey_to_mute.public_key(),
        event_id
    );

    Ok(())
}
