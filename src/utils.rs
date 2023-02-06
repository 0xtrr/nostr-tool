use nostr_sdk::blocking::Client;
use nostr_sdk::prelude::*;

pub fn handle_keys(private_key: Option<String>) -> Result<Keys> {
    // Parse and validate private key
    let keys = match private_key {
        Some(pk) => {
            // create a new identity using the provided private key
            Keys::from_sk_str(pk.as_str())?
        }
        None => {
            // create a new identity with a new keypair
            println!("No private key provided, creating new identity");
            Keys::generate()
        }
    };

    println!("Private key: {}", keys.secret_key()?.to_bech32()?);
    println!("Public key: {}", keys.public_key().to_bech32()?);
    Ok(keys)
}

pub fn create_client(keys: &Keys, relays: Vec<String>, difficulty: u8) -> Result<Client> {
    let opts = Options::new().wait_for_send(true).difficulty(difficulty);
    let client = Client::new_with_opts(keys, opts);
    let relays = relays.iter().map(|url| (url, None)).collect();
    client.add_relays(relays)?;
    client.connect();
    Ok(client)
}

pub fn parse_key(key: String) -> Result<String> {
    // Check if the key is a bech32 encoded key
    let key = if key.starts_with("npub") {
        XOnlyPublicKey::from_bech32(key)?.to_string()
    } else if key.starts_with("nsec") {
        SecretKey::from_bech32(key)?.display_secret().to_string()
    } else if key.starts_with("note") {
        EventId::from_bech32(key)?.to_string()
    } else {
        // If the key is not bech32 encoded, return it as is
        key
    };
    Ok(key)
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Prefix {
    Npub,
    Nsec,
    Note,
}
