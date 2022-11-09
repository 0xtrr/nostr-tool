use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use clap::{Args, Parser, Subcommand};
use nostr_rust::Identity;
use nostr_rust::nostr_client::Client;
use secp256k1::{KeyPair, Secp256k1, XOnlyPublicKey};
use secp256k1::rand::rngs::OsRng;
use serde::Deserialize;

/// Simple CLI application to interact with nostr
#[derive(Parser)]
#[command(name = "nostr-tool")]
#[command(author = "0xtr. <oxtrr@protonmail.com")]
#[command(version = "0.1")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// User private key
    #[arg(short, long)]
    private_key: Option<String>,
    /// Relay to connect to
    #[arg(short, long, action = clap::ArgAction::Append)]
    relay: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Update metadata
    UpdateMetadata(UpdateMetadata),
    /// Send text note
    TextNote(TextNote),
    /// Recommend a relay
    RecommendServer(RecommendServer),
    /// Publish contacts from a CSV file
    PublishContactListCsv(PublishContactListCsv)
}

#[derive(Args)]
struct UpdateMetadata {
    /// Profile name
    #[arg(short, long)]
    name: Option<String>,
    /// About
    #[arg(short, long)]
    about: Option<String>,
    /// Picture URL
    #[arg(short, long)]
    picture: Option<String>,
}

#[derive(Args)]
struct TextNote {
    /// Text note content
    #[arg(short, long)]
    content: String,
    /// Pubkey references
    #[arg(long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
    /// Event references
    #[arg(long, action = clap::ArgAction::Append)]
    etag: Vec<String>,
}

#[derive(Args)]
struct RecommendServer {
    /// Relay URL to recommend
    #[arg(short, long)]
    url: String,
}

#[derive(Args)]
struct PublishContactListCsv {
    /// Path to CSV file. CSV file should be have the following format:
    /// pubkey,relay_url,petname. See example in resources/contact_list.csv
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    // Parse input
    let args: Cli = Cli::parse();

    if args.relay.is_empty() {
        panic!("No relays specified, one relay is required!")
    }
    // Parse and validate private key
    let identity = match args.private_key {
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

    // Post event
    match &args.command {
        Commands::UpdateMetadata(metadata) => {
            // Set metadata
            let name = match &metadata.name {
                Some(name) => Some(name.as_str()),
                None => None,
            };
            let about = match &metadata.about {
                Some(about) => Some(about.as_str()),
                None => None,
            };
            let picture = match &metadata.picture {
                Some(picture) => Some(picture.as_str()),
                None => None,
            };
            client
                .lock()
                .unwrap()
                .set_metadata(&identity, name, about, picture)
                .unwrap();
            println!("Metadata updated");
        }
        Commands::TextNote(text_note) => {
            // Set up tags
            let mut tags: Vec<Vec<String>> = vec![];
            for tag in text_note.ptag.iter() {
                let new_tag = vec![String::from("p"), String::from(tag)];
                tags.push(new_tag);
            }
            for tag in text_note.etag.iter() {
                let new_tag = vec![String::from("e"), String::from(tag)];
                tags.push(new_tag);
            }
            let event = client
                .lock()
                .unwrap()
                .publish_text_note(&identity, &*text_note.content, &tags)
                .unwrap();
            println!("Published text note with id: {}", event.id);
        }
        Commands::RecommendServer(url) => {
            client
                .lock()
                .unwrap()
                .add_recommended_relay(&identity, url.url.as_str())
                .unwrap();
            println!("Relay {} recommended", url.url);
        }
        Commands::PublishContactListCsv(args) => {
            let mut rdr = csv::Reader::from_path(&args.filepath).unwrap();
            let mut contacts: Vec<nostr_rust::nips::nip2::ContactListTag> = vec![];
            for result in rdr.deserialize() {
                let tag: ContactListTag = result.unwrap();
                let clt = nostr_rust::nips::nip2::ContactListTag {
                    key: tag.pubkey,
                    main_relay: tag.relay,
                    surname: tag.petname
                };
                contacts.push(clt);
            }
            client
                .lock()
                .unwrap()
                .set_contact_list(&identity, contacts)
                .unwrap();
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContactListTag {
    /// 32-bytes hex key - the public key of the contact
    pub pubkey: String,
    /// main relay URL
    pub relay: Option<String>,
    /// Petname
    pub petname: Option<String>,
}