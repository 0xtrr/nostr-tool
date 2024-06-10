use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{parse_key_or_id_to_hex_string, Prefix};

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
    if sub_command_args.to_hex {
        // Input is bech32 encoded so we find the hex value
        let hex_key_or_id = parse_key_or_id_to_hex_string(sub_command_args.key.clone()).await?;
        println!("{hex_key_or_id}");
    } else {
        // Input is hex so we bech32 encode it based on the provided prefix value
        let encoded_key: String = match sub_command_args
            .prefix
            .as_ref()
            .expect("Prefix parameter is missing")
        {
            Prefix::Npub => PublicKey::from_str(sub_command_args.key.as_str())?.to_bech32()?,
            Prefix::Nsec => SecretKey::from_str(sub_command_args.key.as_str())?.to_bech32()?,
            Prefix::Note => EventId::from_str(sub_command_args.key.as_str())?.to_bech32()?,
        };
        println!("{encoded_key}");
    }

    Ok(())
}
