use std::str::FromStr;
use std::sync::{Arc, Mutex};
use clap::{Args, Parser, Subcommand};
use nostr_rust::Identity;
use nostr_rust::nostr_client::Client;
use secp256k1::{KeyPair, Secp256k1, XOnlyPublicKey};
use secp256k1::rand::rngs::OsRng;

/// Simple CLI application to interact with nostr
#[derive(Parser)]
#[command(name = "nostr-tool")]
#[command(author = "0xtr. <oxtrr@protonmail.com")]
#[command(version = "0.1")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// User private key
    #[arg(short, long)]
    privatekey: Option<String>,
    /// Relay to connect to
    #[arg(short, long, action = clap::ArgAction::Append)]
    relay: Vec<String>,
    /// Event kind
    #[arg(short, long)]
    kind: u32,
    /// Text note content
    #[arg(short, long)]
    content: Option<String>,
    /// Pubkey references
    #[arg(long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    #[arg(long, action = clap::ArgAction::Append)]
    etag: Vec<String>,
    /// Name value for kind 0 event
    #[arg(long)]
    name: Option<String>,
    /// About value for kind 0 event
    #[arg(long)]
    about: Option<String>,
    /// Picture value to set for kind 0 event
    #[arg(long)]
    picture: Option<String>,
}

fn main() {
    // Parse input
    let args: Cli = Cli::parse();
    // Parse and validate private key
    let identity = match args.privatekey {
        Some(pk) => {
            println!("Using provided private key");
            let identity = Identity::from_str(pk.as_str()).unwrap();
            identity
        }
        None => {
            println!("No private key provided, creating new identity");
            let (secret_key, _) = nostr_rust::keys::get_random_secret_key();
            let identity = Identity::from_str(&secret_key.display_secret().to_string()).unwrap();
            println!("Using new private key {}", identity.secret_key.display_secret().to_string());
            println!("Using new public key {}", identity.public_key.to_string());
            identity
        }
    };
    let str_slice = args.relay.iter().map(String::as_str).collect();
    // Set up relay connection(s)
    let client = Arc::new(Mutex::new(
        Client::new(str_slice).unwrap()
    ));

    // Set up tags
    let mut tags: Vec<Vec<String>> = vec![];
    for tag in args.ptag.iter() {
        let new_tag = vec![String::from("p"), String::from(tag)];
        tags.push(new_tag);
    }
    for tag in args.etag.iter() {
        let new_tag = vec![String::from("e"), String::from(tag)];
        tags.push(new_tag);
    }

    // Post event
    match &args.kind {
        0 => {
            // Set metadata
            let name = match &args.name {
                Some(name) => Some(name.as_str()),
                None => None,
            };
            let about = match &args.about {
                Some(about) => Some(about.as_str()),
                None => None,
            };
            let picture = match &args.picture {
                Some(picture) => Some(picture.as_str()),
                None => None,
            };
            client
                .lock()
                .unwrap()
                .set_metadata(&identity, name, about, picture)
                .unwrap();
        }
        1 => {
            // Create text note
            let content = match &args.content {
                Some(content) => content,
                None => panic!("Content must be set to create a kind 1 event")
            };
            let event = client
                .lock()
                .unwrap()
                .publish_text_note(&identity, content, &tags)
                .unwrap();
            println!("Published text note with id: {}", event.id);
        }
        _ => {
            panic!("Unknown event kind {}", args.kind)
        }
    }
}