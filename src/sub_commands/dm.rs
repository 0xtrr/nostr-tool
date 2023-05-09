use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys, parse_key};

#[derive(Args)]
pub struct SendDirectMessageSubCommand {
    /// Receiver public key. Both hex and bech32 encoded keys are supported.
    #[arg(short, long)]
    receiver: String,
    /// Message to send
    #[arg(short, long)]
    message: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn send(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SendDirectMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays, difficulty_target);

    let hex_pubkey = parse_key(sub_command_args.receiver.clone())?;
    let receiver = XOnlyPublicKey::from_str(&hex_pubkey)?;

    let event_id = client?.send_direct_msg(receiver, sub_command_args.message.clone())?;
    if !sub_command_args.hex {
        println!(
            "Message sent to {}, event id: {}",
            receiver.to_bech32()?,
            event_id.to_bech32()?
        );
    } else {
        println!(
            "Message sent to {}, event id: {}",
            receiver,
            event_id.to_hex()
        );
    }
    Ok(())
}

#[derive(Args)]
pub struct ReadDirectMessageSubCommand {
    /// Sender's public key. Both hex and bech32 encoded keys are supported.
    #[arg(short, long)]
    sender: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub fn read(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &ReadDirectMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays, difficulty_target)?;
    let subscription = Filter::new()
        .pubkey(keys.public_key())
        .kind(Kind::EncryptedDirectMessage)
        .since(Timestamp::now());

    client.subscribe(vec![subscription]);

    let hex_pubkey = parse_key(sub_command_args.sender.clone())?;
    let sender = XOnlyPublicKey::from_str(&hex_pubkey)?;

    client.handle_notifications(|notification| {
        if let RelayPoolNotification::Event(_url, event) = notification {
            if event.kind == Kind::EncryptedDirectMessage {
                match decrypt(&keys.secret_key()?, &event.pubkey, &event.content) {
                    Ok(msg) => {
                        if !sub_command_args.hex {
                            println!(
                                "Message from {}, event id: {}, content: {}",
                                sender.to_bech32()?,
                                event.id.to_bech32()?,
                                msg
                            );
                        } else {
                            println!(
                                "Message from {}, event id: {},  content: {}",
                                sender,
                                event.id.to_hex(), 
                                msg
                            );
                        }
                    }
                    Err(e) => println!("Impossible to decrypt direct message: {e}"),
                }
            }
        }
        Ok(())
    }).expect("error reading direct messages");

    Ok(())

}
