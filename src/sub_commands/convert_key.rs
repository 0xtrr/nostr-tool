use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{parse_key, Prefix};

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

pub fn convert_key(sub_command_args: &ConvertKeySubCommand) {
    if sub_command_args.to_hex {
        let hex_key = &sub_command_args.key.clone();
        let parsed_key = parse_key(hex_key.to_string());
        println!("{parsed_key}");
    } else {
        let encoded_key: String = match sub_command_args
            .prefix
            .as_ref()
            .expect("Prefix parameter is missing")
        {
            Prefix::Npub => XOnlyPublicKey::from_str(&sub_command_args.key)
                .expect("Invalid public key")
                .to_bech32()
                .unwrap(),
            Prefix::Nsec => SecretKey::from_str(&sub_command_args.key)
                .expect("Invalid secret key")
                .to_bech32()
                .unwrap(),
            Prefix::Note => EventId::from_hex(&sub_command_args.key)
                .expect("Invalid event id")
                .to_bech32()
                .unwrap(),
        };
        println!("{encoded_key}");
    }
}
