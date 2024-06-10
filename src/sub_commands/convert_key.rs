use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;
use nostr_sdk::secp256k1::XOnlyPublicKey;

use crate::utils::Prefix;

#[derive(Args)]
pub struct ConvertKeySubCommand {
    /// Pubkey in bech32 or hex format
    #[arg(short, long)]
    key: String,
    /// Bech32 prefix. Only used if you're converting from hex to bech32 encoded keys.
    #[arg(short, long)]
    prefix: Option<Prefix>,
    /// Set to true if you're converting from bech32 to hex
    #[arg(short, long, default_value = "false")]
    to_hex: bool,
}

pub async fn convert_key(sub_command_args: &ConvertKeySubCommand) -> Result<()> {
    let unknown_key = &sub_command_args.key.clone();

    let hex_key = if unknown_key.starts_with("npub") {
        PublicKey::from_bech32(unknown_key.clone())
            .unwrap()
            .to_string()
    } else if unknown_key.starts_with("nsec") {
        SecretKey::from_bech32(unknown_key)?
            .display_secret()
            .to_string()
    } else if unknown_key.starts_with("note") {
        EventId::from_bech32(unknown_key)?.to_hex()
    } else {
        // If the key is not bech32 encoded, return it as is
        unknown_key.clone()
    };

    if sub_command_args.to_hex {
        println!("{hex_key}");
    } else {
        let encoded_key: String = match sub_command_args
            .prefix
            .as_ref()
            .expect("Prefix parameter is missing")
        {
            Prefix::Npub => XOnlyPublicKey::from_str(hex_key.as_str())?.to_string(),
            Prefix::Nsec => SecretKey::from_str(hex_key.as_str())?.to_bech32()?,
            Prefix::Note => EventId::from_hex(hex_key)?.to_bech32()?,
        };
        println!("{encoded_key}");
    }

    Ok(())
}
