use std::str::FromStr;
use std::sync::{Arc, Mutex};
use nostr_rust::Identity;
use nostr_rust::nostr_client::Client;

pub fn handle_identity(private_key: Option<String>) -> Identity {
    // Parse and validate private key
    let identity = match private_key {
        Some(pk) => {
            let identity = Identity::from_str(pk.as_str()).unwrap();
            identity
        }
        None => {
            println!("No private key provided, creating new identity");
            let (secret_key, _) = nostr_rust::keys::get_random_secret_key();
            let identity = Identity::from_str(&secret_key.display_secret().to_string()).unwrap();
            println!(
                "New private key {}",
                identity.secret_key.display_secret().to_string()
            );
            println!("New public key {}", identity.public_key.x_only_public_key().0.to_string());
            identity
        }
    };
    identity
}

pub fn create_client(relays: Vec<String>) -> Arc<Mutex<Client>> {
    // Set up relay connection(s)
    let str_slice = relays.iter().map(String::as_str).collect();
    let connection_result = Client::new(str_slice);
    let client = match connection_result {
        Ok(connected_client) => Arc::new(Mutex::new(connected_client)),
        Err(e) => panic!("{}", e)
    };
    client
}