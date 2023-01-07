use clap::{Args};

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;
use nostr_tool::utils::{hex_to_bech32, parse_key, Prefix};

#[derive(Args)]
pub struct DeleteEventSubCommand {
    /// Event id to delete
    #[arg(short, long)]
    event_id: String,
    /// Reason for deleting the events
    #[arg(short, long)]
    reason: Option<String>,
}

pub fn delete(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u16,
    sub_command_args: &DeleteEventSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    let event_id_to_delete_hex = parse_key(sub_command_args.event_id.clone());
    let bech32_encoded_id = hex_to_bech32(Prefix::Note, event_id_to_delete_hex.clone());

    match &sub_command_args.reason {
        Some(reason) => {
            let result = client
                .lock()
                .unwrap()
                .delete_event_with_reason(
                    &identity,
                    event_id_to_delete_hex.as_str(),
                    reason,
                    difficulty_target,
                );
            match result {
                Ok(_) => println!("Deleted event with id: {}", bech32_encoded_id),
                Err(e) => eprintln!("{}", e)
            }
        }
        None => {
            let result = client
                .lock()
                .unwrap()
                .delete_event(
                    &identity,
                    event_id_to_delete_hex.as_str(),
                    difficulty_target,
                );
            match result {
                Ok(_) => {
                    println!("Deleted event with id: {}", bech32_encoded_id)
                },
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}