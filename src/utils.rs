use std::str::FromStr;
use std::sync::{Arc, Mutex};
use nostr_rust::Identity;
use nostr_rust::nostr_client::Client;
use bech32::{FromBase32, ToBase32, Variant};

pub fn handle_identity(private_key: Option<String>) -> Identity {
    // Parse and validate private key
    let identity = match private_key {
        Some(pk) => {
            // ensure we use hex key further down in the APIs
            let parsed_key = parse_key(pk);
            // create a new identity using the provided private key
            match Identity::from_str(parsed_key.as_str()) {
                Ok(identity) => identity,
                Err(err) => panic!("Error creating identity: {}", err),
            }
        }
        None => {
            // create a new identity with a new keypair
            println!("No private key provided, creating new identity");
            generate_new_identity()
        }
    };

    let priv_key = identity.secret_key.clone().display_secret().to_string();
    let bech32_encoded_private_key = bech32_encode_key(Prefix::Nsec, priv_key);

    let public_key = identity.public_key_str.clone();
    let bech32_encoded_public_key = bech32_encode_key(Prefix::Npub, public_key);
    println!("Private key: {}", bech32_encoded_private_key);
    println!("Public key: {}", bech32_encoded_public_key);
    identity
}

pub fn generate_new_identity() -> Identity {
    let (secret_key, _) = nostr_rust::keys::get_random_secret_key();
    let identity = match Identity::from_str(&secret_key.display_secret().to_string()) {
        Ok(identity) => identity,
        Err(err) => panic!("Error creating identity: {}", err),
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

pub fn parse_key(key: String) -> String {
    // Check if the key is a bech32 encoded key
    if key.starts_with("npub") || key.starts_with("nsec") {
        let (_, data, _) = bech32::decode(key.as_str()).expect("could not decode data");
        let hex_key = Vec::<u8>::from_base32(&data).expect("could not convert data to Vec<u8>");
        hex::encode(hex_key)
    } else {
        // If the key is not bech32 encoded, return it as is
        key
    }
}

pub enum Prefix {
    Npub,
    Nsec
}

pub fn bech32_encode_key(kind: Prefix, pubkey: String) -> String{
    let hrp = match kind {
        Prefix::Npub => "npub",
        Prefix::Nsec => "nsec",
    };

    let encoded = bech32::encode(
        hrp,
        hex::decode(pubkey).unwrap().to_base32(),
        Variant::Bech32
    ).unwrap();
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bech32_encoded_public_key() {
        let bech32_encoded_key = String::from("npub1ktt8phjnkfmfrsxrgqpztdjuxk3x6psf80xyray0l3c7pyrln49qhkyhz0");
        let hex_key = String::from("b2d670de53b27691c0c3400225b65c35a26d06093bcc41f48ffc71e0907f9d4a");
        let parsed_key = parse_key(bech32_encoded_key);
        assert_eq!(parsed_key, hex_key);
    }

    #[test]
    fn parse_hex_public_key() {
        let hex_key = String::from("b2d670de53b27691c0c3400225b65c35a26d06093bcc41f48ffc71e0907f9d4a");
        let parsed_key = parse_key(hex_key.clone());
        assert_eq!(parsed_key, hex_key);
    }

    #[test]
    fn parse_bech32_encoded_private_key() {
        let bech32_encoded_key = String::from("nsec1hdeqm0y8vgzuucqv4840h7rlpy4qfu928ulxh3dzj6s2nqupdtzqagtew3");
        let hex_key = String::from("bb720dbc876205ce600ca9eafbf87f092a04f0aa3f3e6bc5a296a0a983816ac4");
        let parsed_key = parse_key(bech32_encoded_key);
        assert_eq!(parsed_key, hex_key);
    }
}