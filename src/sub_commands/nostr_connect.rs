use std::str::FromStr;

use futures::executor::block_on;

use clap::Args;
use nostr_sdk::nips::nip46::NostrConnectMetadata;
use nostr_sdk::nips::nip46::{Message, Request};
use nostr_sdk::prelude::*;
use nostr_sdk::secp256k1::schnorr::Signature;

use std::io;
use std::io::Write;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct ConnectSignerSubCommand {
    /// Text note content
    #[arg(short, long)]
    connect_url: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

#[derive(Args)]
pub struct ConnectAppSubCommand {
    /// Text note content
    #[arg(short, long)]
    url: Option<String>,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}
pub fn app(
    private_key: Option<String>,
    relays: Vec<String>,
    sub_command_args: &ConnectAppSubCommand,
) -> Result<()> {
    let keys = handle_keys(private_key, sub_command_args.hex)?;
    println!(":{}:", relays[0]);

    let mut nostr_connect_uri: NostrConnectURI =
        NostrConnectURI::new(keys.public_key(), Url::parse(&relays[0])?, "Nostr Tool");

    // Add app url if defined
    if let Some(url) = &sub_command_args.url {
        nostr_connect_uri = nostr_connect_uri.url(Url::parse(url)?);
    }

    let client = create_client(&keys, relays, 0)?;

    println!("\n###############################################\n");
    println!("Nostr Connect URI: {nostr_connect_uri}");
    println!("\n###############################################\n");

    client.connect();

    // Listen for connect ACK
    let signer_pubkey = block_on(get_signer_pubkey(&client));
    println!("Received signer pubkey: {signer_pubkey}");

    println!("\n###############################################\n");

    /*
    let msg = Message::request(Request::GetPublicKey);
    let res = block_on(get_response(&client, signer_pubkey, msg))?;
    if let Response::GetPublicKey(pubkey) = res {
        println!("Received pubeky {pubkey}");
        println!("\n###############################################\n");
    }
    */

    // compose unsigned event
    let unsigned_event = EventBuilder::new_text_note("Hello world from Nostr Tool", &[])
        .to_unsigned_event(signer_pubkey);
    let msg = Message::request(Request::SignEvent(unsigned_event.clone()));
    println!("Sending Sign: {:?}", msg);
    let res = block_on(get_response(&client, signer_pubkey, msg))?;
    if let Response::SignEvent(sig) = res {
        let event = unsigned_event.add_signature(sig)?;
        let id = client.send_event(event)?;
        println!("Published event {id}");
        println!("\n###############################################\n");
    }

    Ok(())
}

pub fn signer(
    private_key: Option<String>,
    sub_command_args: &ConnectSignerSubCommand,
) -> Result<()> {
    println!("Running Nostr Connect Signer");
    println!("\n###############################################\n");
    let parsed_url = Url::parse(&sub_command_args.connect_url).unwrap();

    let app_pubkey = parsed_url.domain().unwrap();
    let relay = parsed_url
        .query_pairs()
        .find(|(k, _)| k == "relay")
        .unwrap()
        .1;
    let metadata = parsed_url
        .query_pairs()
        .find(|(k, _)| k == "metadata")
        .unwrap()
        .1;

    let metadata: NostrConnectMetadata = serde_json::from_str(&metadata)?;

    println!("Decoded Nostr connect uri");
    println!("App Pubkey: {app_pubkey:?}");
    println!("Relay: {relay}");
    println!("App Metadata: {:#?}", metadata);
    println!("\n###############################################\n");

    let keys = handle_keys(private_key, sub_command_args.hex)?;
    println!("\n###############################################\n");

    let client = create_client(&keys, vec![relay.to_string()], 0)?;

    // Connect ACK message
    let msg = Message::request(Request::Connect(keys.public_key()));

    let keys = client.keys();

    // Subscribe to Nostr connect messages
    client.subscribe(vec![Filter::new()
        .pubkey(keys.public_key())
        .kind(Kind::NostrConnect)]);

    // Connect event
    let event = EventBuilder::nostr_connect(&keys, XOnlyPublicKey::from_str(app_pubkey)?, msg)?
        .to_event(&keys)?;
    client.send_event(event)?;

    // Request::Connect(XOnlyPublicKey::from_str(hash)?);
    // let res = block_on(get_response(
    //     &client,
    //    XOnlyPublicKey::from_str(hash)?,
    //    msg.clone(),
    // ))?;
    // println!("res: {res:?}");
    block_on(get_request(&client, XOnlyPublicKey::from_str(app_pubkey)?))?;

    Ok(())
}

async fn get_request(
    client: &nostr_sdk::blocking::Client,
    app_pubkey: XOnlyPublicKey,
) -> Result<()> {
    let keys = client.keys();

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        if let RelayPoolNotification::Event(_url, event) = notification {
            if event.kind == Kind::NostrConnect {
                match decrypt(&keys.secret_key()?, &event.pubkey, &event.content) {
                    Ok(msg) => {
                        let msg = Message::from_json(msg)?;

                        //println!("New message received: {msg:#?}");
                        //println!("\n###############################################\n");

                        if let Message::Request { id, method, params } = &msg {
                            let msg = match method.as_str() {
                                "sign_event" => {
                                    let event = UnsignedEvent::from_json(params[0].to_string())?;
                                    println!(
                                        "New Event received: {}",
                                        serde_json::to_string_pretty(&event)?
                                    );
                                    println!("\n###############################################\n");

                                    match get_user_confirmation("Would you like to sign?") {
                                        Ok(true) => {
                                            let signed_event = event.sign(&keys)?;

                                            Message::Response {
                                                id: id.to_string(),
                                                result: Some(
                                                    serde_json::from_str(&signed_event.as_json())
                                                        .unwrap(),
                                                ),
                                                error: None,
                                            }
                                        }
                                        Ok(false) => Message::Response {
                                            id: id.to_string(),
                                            result: None,
                                            error: Some("Declined to sign".to_string()),
                                        },
                                        Err(err) => Message::Response {
                                            id: id.to_string(),
                                            result: None,
                                            error: Some(format!("{}", err)),
                                        },
                                    }
                                }
                                _ => todo!(),
                            };

                            // Create response event
                            let event =
                                EventBuilder::nostr_connect(&client.keys(), app_pubkey, msg)?
                                    .to_event(&keys)?;

                            client.send_event(event)?;
                        }
                    }
                    Err(e) => eprintln!("Impossible to decrypt NIP46 message: {e}"),
                }
            }
        }
    }

    client.unsubscribe();

    panic!("");
}

async fn get_response(
    client: &nostr_sdk::blocking::Client,
    signer_pubkey: XOnlyPublicKey,
    msg: Message,
) -> Result<Response> {
    let keys = client.keys();
    let req_id = msg.id();
    let req = msg.to_request()?;

    let event = EventBuilder::nostr_connect(&keys, signer_pubkey, msg)?.to_event(&keys)?;
    client.send_event(event)?;

    client.subscribe(vec![Filter::new()
        .pubkey(keys.public_key())
        .kind(Kind::NostrConnect)]);

    println!("Waiting for response to request: {}", req_id);
    println!("\n###############################################\n");

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        if let RelayPoolNotification::Event(_url, event) = notification {
            if event.kind == Kind::NostrConnect {
                match decrypt(&keys.secret_key()?, &event.pubkey, &event.content) {
                    Ok(msg) => {
                        let msg = Message::from_json(msg)?;

                        println!("New message received: {msg:#?}");
                        println!("\n###############################################\n");

                        if let Message::Response { id, result, error } = &msg {
                            if &req_id == id {
                                if let Some(result) = result {
                                    let res = match req {
                                        Request::SignEvent(_) => {
                                            let sig: Value =
                                                serde_json::from_value(result.to_owned())?;
                                            let sig = sig.get("sig").unwrap().as_str().unwrap();
                                            Response::SignEvent(Signature::from_str(sig).unwrap())
                                        }
                                        Request::GetPublicKey => {
                                            let pubkey = serde_json::from_value(result.to_owned())?;
                                            Response::GetPublicKey(pubkey)
                                        }
                                        _ => todo!(),
                                    };
                                    client.unsubscribe();
                                    return Ok(res);
                                }

                                if let Some(error) = error {
                                    client.unsubscribe();
                                    panic!("Error response {error}");
                                }

                                break;
                            }
                        } else {
                            println!("Makes no sense");
                        }
                    }
                    Err(e) => eprintln!("Impossible to decrypt NIP46 message: {e}"),
                }
            }
        }
    }

    client.unsubscribe();

    panic!("");
}

async fn get_signer_pubkey(client: &blocking::Client) -> XOnlyPublicKey {
    client.subscribe(vec![Filter::new()
        .pubkey(client.keys().public_key())
        .kind(Kind::NostrConnect)
        .since(Timestamp::now())]);

    loop {
        let mut notifications = client.notifications();
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event(_url, event) = notification {
                if event.kind == Kind::NostrConnect {
                    match decrypt(
                        &client.keys().secret_key().unwrap(),
                        &event.pubkey,
                        &event.content,
                    ) {
                        Ok(msg) => {
                            let msg = Message::from_json(msg).unwrap();
                            if let Ok(Request::Connect(pubkey)) = msg.to_request() {
                                client.unsubscribe();
                                return pubkey;
                            }
                        }
                        Err(e) => eprintln!("Impossible to decrypt NIP46 message: {e}"),
                    }
                }
            }
        }
    }
}

fn get_user_confirmation(prompt: &str) -> Result<bool> {
    // Prompt for user input
    print!("{} (y/n): ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            // Action to be taken if user confirms (e.g. "yes" input)
            println!("Action confirmed!");
            return Ok(true);
        }
        "n" | "no" => {
            // Action to be taken if user declines (e.g. "no" input)
            println!("Action declined.");
            return Ok(false);
        }
        _ => {
            // Action to be taken if user enters an invalid input
            println!("Invalid input.");
        }
    }
    Ok(false)
}
