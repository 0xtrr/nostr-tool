use std::str::FromStr;
use std::time::Duration;

use nostr_sdk::prelude::*;

pub async fn parse_private_key(private_key: Option<String>, print_keys: bool) -> Result<Keys> {
    // Parse and validate private key
    let keys = match private_key {
        Some(pk) => {
            // Ensure we get private key in hex format
            let hex_priv_key = parse_key_or_id(pk).await?;
            // create a new identity using the provided private key
            let secret_key = SecretKey::from_str(hex_priv_key.as_str())?;
            Keys::new(secret_key)
        }
        None => {
            // create a new identity with a new keypair
            if print_keys {
                println!("No private key provided, creating new identity");
            }
            Keys::generate()
        }
    };

    if print_keys {
        println!("Private key:");
        println!("{}", keys.secret_key()?.to_bech32()?);
        println!("{}", keys.secret_key()?.display_secret());

        println!("Public key:");
        println!("{}", keys.public_key().to_bech32()?);
        println!("{}", keys.public_key());
    }

    Ok(keys)
}

// Creates the websocket client that is used for communicating with relays
pub async fn create_client(keys: &Keys, relays: Vec<String>, difficulty: u8) -> Result<Client> {
    let opts = Options::new()
        .send_timeout(Some(Duration::from_secs(15)))
        .wait_for_send(true)
        .difficulty(difficulty);
    let client = Client::with_opts(keys, opts);
    client.add_relays(relays).await?;
    client.connect().await;
    Ok(client)
}

pub async fn parse_key_or_id(input: String) -> Result<String, Box<dyn std::error::Error>> {
    if is_bech32(input.as_str()) {
        let decoded = bech32::decode(input.as_str()).unwrap();
        let data = hex::encode(decoded.1);
        Ok(data)
    } else {
        Ok(input)
    }
}

fn is_bech32(s: &str) -> bool {
    s.starts_with("npub") || s.starts_with("nsec") || s.starts_with("note")
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Prefix {
    Npub,
    Nsec,
    Note,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Keyformat {
    Hex,
    Bech32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_key_hex_input() {
        let hex_key =
            String::from("f4deaad98b61fa24d86ef315f1d5d57c1a6a533e1e87e777e5d0b48dcd332cdb");
        let result = parse_key_or_id(hex_key.clone()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), hex_key);
    }

    #[tokio::test]
    async fn test_parse_key_bech32_note_input() {
        let bech32_note_id =
            String::from("note1h445ule4je70k7kvddate8kpsh2fd6n77esevww5hmgda2qwssjsw957wk");

        let result = parse_key_or_id(bech32_note_id).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            String::from("bd6b4e7f35967cfb7acc6b7abc9ec185d496ea7ef6619639d4bed0dea80e8425")
        );
    }

    #[tokio::test]
    async fn test_parse_bech32_public_key_input() {
        let bech32_encoded_key =
            String::from("npub1ktt8phjnkfmfrsxrgqpztdjuxk3x6psf80xyray0l3c7pyrln49qhkyhz0");
        let result = parse_key_or_id(bech32_encoded_key).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            String::from("b2d670de53b27691c0c3400225b65c35a26d06093bcc41f48ffc71e0907f9d4a")
        );
    }

    #[tokio::test]
    async fn test_parse_bech32_private_key() {
        let bech32_encoded_key =
            String::from("nsec1hdeqm0y8vgzuucqv4840h7rlpy4qfu928ulxh3dzj6s2nqupdtzqagtew3");
        let result = parse_key_or_id(bech32_encoded_key).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            String::from("bb720dbc876205ce600ca9eafbf87f092a04f0aa3f3e6bc5a296a0a983816ac4")
        );
    }
}
