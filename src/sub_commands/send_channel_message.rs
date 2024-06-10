use crate::utils::{create_client, handle_keys};
use clap::Args;
use nostr_sdk::prelude::*;

#[derive(Args)]
pub struct SendChannelMessageSubCommand {
    /// Channel id to send message to (must be hex)
    #[arg(short, long)]
    channel_id: String,
    /// Message content
    #[arg(short, long)]
    message: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

pub async fn send_channel_message(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SendChannelMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    // Process keypair and create a nostr client
    let keys = handle_keys(private_key, true).await?;
    let client = create_client(&keys, relays.clone(), difficulty_target).await?;

    let ch_id: EventId = EventId::from_hex(sub_command_args.channel_id.clone()).unwrap();

    let event_id = client.send_channel_msg(
        ch_id,
        Url::parse(relays[0].as_str())?,
        sub_command_args.message.clone(),
    ).await?;
    println!(
        "Public channel message sent with id: {}",
        event_id.to_bech32()?
    );

    Ok(())
}