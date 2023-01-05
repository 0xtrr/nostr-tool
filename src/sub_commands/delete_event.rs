use clap::{Args};

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;

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

    match &sub_command_args.reason {
        Some(reason) => {
            let result = client
                .lock()
                .unwrap()
                .delete_event_with_reason(
                    &identity,
                    sub_command_args.event_id.as_str(),
                    reason,
                    difficulty_target,
                );
            match result {
                Ok(_) => println!("Deleted event with id: {}", &sub_command_args.event_id),
                Err(e) => eprintln!("{}", e)
            }
        }
        None => {
            let result = client
                .lock()
                .unwrap()
                .delete_event(
                    &identity,
                    sub_command_args.event_id.as_str(),
                    difficulty_target,
                );
            match result {
                Ok(_) => println!("Deleted event with id: {}", &sub_command_args.event_id),
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}