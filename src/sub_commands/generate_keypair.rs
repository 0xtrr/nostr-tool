use clap::Args;

use nostr_tool::utils::{hex_to_bech32, generate_new_identity, Prefix};

#[derive(Args)]
pub struct GenerateKeypairSubCommand {
    /// Print keys in hex. Defaults to printing bech32 encoded keys.
    #[arg(short, long, default_value = "false")]
    print_hex: bool,
}

pub fn get_new_keypair(sub_command_args: &GenerateKeypairSubCommand) {
    let identity = generate_new_identity();
    if sub_command_args.print_hex {
        println!("Private key: {}", identity.secret_key.clone().display_secret().to_string());
        println!("Public key: {}", identity.public_key_str)
    } else {
        let priv_key = identity.secret_key.clone().display_secret().to_string();
        let bech32_encoded_private_key = hex_to_bech32(Prefix::Nsec, priv_key);

        let public_key = identity.public_key_str.clone();
        let bech32_encoded_public_key = hex_to_bech32(Prefix::Npub, public_key);

        println!("Private key: {}", bech32_encoded_private_key);
        println!("Public key: {}", bech32_encoded_public_key);
    }
}