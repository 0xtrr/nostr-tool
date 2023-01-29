use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::{*, Tag};

use crate::utils::{self, create_client, handle_keys};

#[derive(Args)]
pub struct TextNoteSubCommand {
    /// Text note content
    #[arg(short, long)]
    content: String,
    /// Pubkey references. Both hex and bech32 encoded keys are supported.
    #[arg(long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    /// Event references
    #[arg(long, action = clap::ArgAction::Append)]
    etag: Vec<String>,
    /// Seconds till expiration (NIP-40)
    #[arg(long)]
    expiration: Option<u64>,
}

pub fn broadcast_textnote(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &TextNoteSubCommand,
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key);
    let client = create_client(&keys, relays, difficulty_target);

    // Set up tags
    let mut tags: Vec<Tag> = vec![];
    for ptag in sub_command_args.ptag.iter() {
        // Parse pubkey to ensure we're sending hex keys
        let pubkey_hex = utils::parse_key(ptag.clone());
        let pubkey = XOnlyPublicKey::from_str(&pubkey_hex).expect("Invalid public key");
        tags.push(Tag::PubKey(pubkey, None));
    }
    for etag in sub_command_args.etag.iter() {
        let event_id = EventId::from_hex(etag).expect("Invalid event id");
        tags.push(Tag::Event(event_id, None, None));
    }
    if let Some(expiration) = sub_command_args.expiration {
        let new_tag = Tag::Expiration(Timestamp::now().as_u64() + expiration);
        tags.push(new_tag);
    }

    match client.publish_text_note(sub_command_args.content.clone(), &tags) {
        Ok(id) => println!("Published text note with id: {}", id.to_bech32().unwrap()),
        Err(e) => eprintln!("{e}"),
    }

}
