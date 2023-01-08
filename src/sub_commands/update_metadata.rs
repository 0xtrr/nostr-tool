use clap::{Args};
use nostr_rust::events::EventPrepare;
use nostr_rust::utils::get_timestamp;
use serde_json::json;

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;
use nostr_tool::utils::{hex_to_bech32, Prefix};

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
    #[arg(long)]
    lud06: Option<String>,
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

    let mut json_body = json!({});

    // Name
    if let Some(name) = &sub_command_args.name {
        json_body["name"] = json!(name.as_str())
    }

    // About
    if let Some(about) = &sub_command_args.about {
        json_body["about"] = json!(about.as_str())
    }

    // Picture URL
    if let Some(picture_url) = &sub_command_args.picture {
        json_body["picture"] = json!(picture_url.as_str())
    };

    // NIP-05 identifier
    if let Some(nip05_identifier) = &sub_command_args.nip05 {
        // Check if the nip05 is valid
        match nostr_rust::nips::nip5::check_validity(
            nip05_identifier.as_str(),
            identity.public_key_str.clone().as_str(),
        )  {
            Ok(valid) => {
                if !valid {
                    panic!("NIP-05 identifier is not valid")
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        json_body["nip05"] = json!(nip05_identifier)
    }

    // LUD-06 string
    if let Some(lud06) = &sub_command_args.lud06 {
        json_body["lud06"] = json!(lud06)
    }


    let event = EventPrepare {
        pub_key: identity.public_key_str.clone(),
        created_at: get_timestamp(),
        kind: 0,
        tags: vec![],
        content: json_body.to_string(),
    }.to_event(&identity, difficulty_target);

    let result = client.lock().unwrap().publish_event(&event);
    match result {
        Ok(_) => {
            let bech32_encoded_id = hex_to_bech32(Prefix::Note, event.id.clone());
            println!("Metadata updated ({})", bech32_encoded_id)
        },
        Err(e) => eprintln!("{}", e)
    }
}