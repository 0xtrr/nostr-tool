use clap::Args;

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;
use nostr_tool::utils;

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

    let key_clone = sub_command_args.receiver.clone();
    let hex_pubkey = utils::parse_key(key_clone);

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
        Ok(event) => println!(
            "Message sent to {}, event id: {}",
            sub_command_args.receiver, event.id
        ),
        Err(e) => eprintln!("{}", e)
    }
}