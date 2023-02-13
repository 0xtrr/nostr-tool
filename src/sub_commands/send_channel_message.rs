use clap::Args;
use nostr_sdk::prelude::*;
use crate::utils::{create_client, handle_keys, parse_key};

#[derive(Args)]
pub struct SendChannelMessageSubCommand {
    /// Channel id to send message to
    #[arg(short, long)]
    channel_id: String,
    /// Message content
    #[arg(short, long)]
    message: String,
}

pub fn send_channel_message(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SendChannelMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key)?;
    let client = create_client(&keys, relays.clone(), difficulty_target)?;

    let hex_channel_id: String = parse_key(sub_command_args.channel_id.clone())?;
    let ch_id: ChannelId = ChannelId::from_hex(hex_channel_id)?;

    let event_id = client.send_channel_msg(ch_id, Some(Url::parse(relays[0].as_str())?), sub_command_args.message.clone())?;
    println!("Public channel message sent with id: {}", event_id.to_bech32()?);

    Ok(())
}
