use std::io;
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;

use clap::Args;
use futures::executor::block_on;

use nostr_sdk::blocking::Client;
use nostr_sdk::nips::nip46::{Message, NostrConnectMetadata, Request};
use nostr_sdk::prelude::*;

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

    let mut nostr_connect_uri: NostrConnectURI =
        NostrConnectURI::new(keys.public_key(), Url::parse(&relays[0])?, "Nostr Tool");

    // Add app url if defined
    if let Some(url) = &sub_command_args.url {
        nostr_connect_uri = nostr_connect_uri.url(Url::parse(url)?);
    }

    let relay_url = Url::from_str(&relays[0])?;

    let signer = RemoteSigner::new(relay_url, None);

    let client = Client::with_remote_signer(&keys, signer);

    println!("\n###############################################\n");
    println!("Nostr Connect URI: {nostr_connect_uri}");
    println!("\n###############################################\n");

    client.add_relay(relays[0].clone(), None)?;
    client.connect();

    println!("Requesting pub key");

    // Get Signer Pubkey
    client.req_signer_public_key(Some(Duration::from_secs(180)))?;

    loop {
        print!("Content: ");
        let mut content = String::new();
        // io::stdin()
        //    .read_line(&mut content)
        //     .expect("Failed to read line");

        content = "hello world".to_string();
        let id = client.publish_text_note(content, &[])?;

        println!("Broadcasted Event: {}", id.to_hex());
        println!("\n###############################################\n");
    }
}

pub fn signer(
    private_key: Option<String>,
    sub_command_args: &ConnectSignerSubCommand,
) -> Result<()> {
    println!("Running Nostr Connect Signer");
    println!("\n###############################################\n");
    let parsed_url = Url::parse(&sub_command_args.connect_url)?;
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

                                            // let result =

                                            Message::Response {
                                                id: id.to_string(),
                                                result: Some(Value::String(
                                                    signed_event.sig.to_string(),
                                                )),
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
