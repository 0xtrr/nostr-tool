use clap::{Args};

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;
use nostr_tool::utils;

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
}

pub fn broadcast_textnote(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u16,
    sub_command_args: &TextNoteSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    // Set up tags
    let mut tags: Vec<Vec<String>> = vec![];
    for ptag in sub_command_args.ptag.iter() {
        // Parse pubkey to ensure we're sending hex keys
        let pubkey_hex = utils::parse_key(ptag.clone());
        let new_tag = vec![String::from("p"), String::from(pubkey_hex)];
        tags.push(new_tag);
    }
    for etag in sub_command_args.etag.iter() {
        let new_tag = vec![String::from("e"), String::from(etag)];
        tags.push(new_tag);
    }
    let result = client
        .lock()
        .unwrap()
        .publish_text_note(&identity, &sub_command_args.content, &tags, difficulty_target);
    match result {
        Ok(event) => println!("Published text note with id: {}", event.id),
        Err(e) => eprintln!("{}", e)
    }
}

