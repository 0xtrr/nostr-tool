use clap::Args;

use nostr_tool::utils::{hex_to_bech32, parse_key, Prefix};

#[derive(Args)]
pub struct ConvertKeySubCommand {
    /// Pubkey in bech32 or hex format
    #[arg(short, long)]
    key: String,
    /// Bech32 prefix. Only used if you're converting from hex to bech32 encoded keys.
    #[arg(short, long)]
    prefix: Option<Prefix>,
    /// Set to true if you're converting from bech32 to hex
    #[arg(short, long, default_value = "false" )]
    to_hex: bool,
}

pub fn convert_key(sub_command_args: &ConvertKeySubCommand) {
    if sub_command_args.to_hex.clone() {
        let hex_key = &sub_command_args.key.clone();
        let parsed_key = parse_key(hex_key.to_string());
        println!("{}", parsed_key);
    } else {
        let hrp = match sub_command_args.prefix.clone() {
            Some(prefix) => prefix,
            None => panic!("Prefix parameter is missing")
        };
        let encoded_key = hex_to_bech32(
            hrp,
            sub_command_args.key.clone()
        );
        println!("{}", encoded_key);
    }
}