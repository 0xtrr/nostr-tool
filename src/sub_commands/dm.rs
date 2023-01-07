use clap::Args;

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;
use nostr_tool::utils::{hex_to_bech32, parse_key, Prefix};

#[derive(Args)]
pub struct SendDirectMessageSubCommand {
    /// Receiver public key. Both hex and bech32 encoded keys are supported.
    #[arg(short, long)]
    receiver: String,
    /// Message to send
    #[arg(short, long)]
    message: String,
}

pub fn send(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u16,
    sub_command_args: &SendDirectMessageSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    let hex_pubkey = parse_key(sub_command_args.receiver.clone());
    let receiver_bech32_encoded_pubkey = hex_to_bech32(Prefix::Npub, hex_pubkey.clone());

    let result = client
        .lock()
        .unwrap()
        .send_private_message(
            &identity,
            hex_pubkey.as_str(),
            &sub_command_args.message,
            difficulty_target,
        );
    match result {
        Ok(event) => {
            let bech32_encoded_note_id = hex_to_bech32(Prefix::Note, event.id);
            println!(
                "Message sent to {}, event id: {}",
                receiver_bech32_encoded_pubkey, bech32_encoded_note_id
            )
        },
        Err(e) => eprintln!("{}", e)
    }
}