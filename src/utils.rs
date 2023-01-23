use nostr_sdk::blocking::Client;
use nostr_sdk::prelude::*;

pub fn handle_keys(private_key: Option<String>) -> Keys {
    // Parse and validate private key
    let keys = match private_key {
        Some(pk) => {
            // create a new identity using the provided private key
            match Keys::from_sk_str(pk.as_str()) {
                Ok(keys) => keys,
                Err(err) => panic!("Error creating identity: {err}"),
            }
        }
        None => {
            // create a new identity with a new keypair
            println!("No private key provided, creating new identity");
            Keys::generate()
        }
    };

    println!(
        "Private key: {}",
        keys.secret_key().unwrap().to_bech32().unwrap()
    );
    println!("Public key: {}", keys.public_key().to_bech32().unwrap());
    keys
}

pub fn create_client(keys: &Keys, relays: Vec<String>, difficulty: u8) -> Client {
    let opts = Options::new().wait_for_send(true).difficulty(difficulty);
    let client = Client::new_with_opts(keys, opts);
    let relays = relays.iter().map(|url| (url, None)).collect();
    client.add_relays(relays).expect("Impossible to add relays");
    client.connect();
    client
}

pub fn parse_key(key: String) -> String {
    // Check if the key is a bech32 encoded key
    if key.starts_with("npub") {
        XOnlyPublicKey::from_bech32(key)
            .expect("Invalid bech32 public key")
            .to_string()
    } else if key.starts_with("nsec") {
        SecretKey::from_bech32(key)
            .expect("Invalid bech32 secret key")
            .display_secret()
            .to_string()
    } else if key.starts_with("note") {
        EventId::from_bech32(key)
            .expect("Invalid bech32 note")
            .to_string()
    } else {
        // If the key is not bech32 encoded, return it as is
        key
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Prefix {
    Npub,
    Nsec,
    Note,
}
