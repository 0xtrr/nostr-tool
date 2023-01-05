use clap::{Args};

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;

#[derive(Args)]
pub struct UpdateMetadataSubCommand {
    /// Profile name
    #[arg(short, long)]
    name: Option<String>,
    /// About
    #[arg(short, long)]
    about: Option<String>,
    /// Picture URL
    #[arg(short, long)]
    picture: Option<String>,
    #[arg(long)]
    nip05: Option<String>,
}

pub fn update_metadata(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u16,
    sub_command_args: &UpdateMetadataSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    // Set metadata
    let name = match &sub_command_args.name {
        Some(name) => Some(name.as_str()),
        None => None,
    };
    let about = match &sub_command_args.about {
        Some(about) => Some(about.as_str()),
        None => None,
    };
    let picture = match &sub_command_args.picture {
        Some(picture) => Some(picture.as_str()),
        None => None,
    };
    let nip05 = match &sub_command_args.nip05 {
        Some(nip05) => Some(nip05.as_str()),
        None => None,
    };

    let result = client
        .lock()
        .unwrap()
        .set_metadata(&identity, name, about, picture, nip05, difficulty_target);
    match result {
        Ok(event) => println!("Metadata updated ({})", event.id),
        Err(e) => eprintln!("{}", e)
    }
}