use std::{collections::HashMap, str::FromStr, time::Duration};

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::create_client;

#[derive(Args)]
pub struct ListEventsSubCommand {
    /// Ids
    #[arg(short, long, action = clap::ArgAction::Append)]
    ids: Option<Vec<String>>,
    /// Authors
    #[arg(short, long, action = clap::ArgAction::Append)]
    authors: Option<Vec<String>>,
    /// Kinds
    #[arg(short, long, action = clap::ArgAction::Append)]
    kinds: Option<Vec<u64>>,
    /// e tag
    #[arg(short, long, action = clap::ArgAction::Append)]
    e: Option<Vec<String>>,
    /// p tag
    #[arg(short, long, action = clap::ArgAction::Append)]
    p: Option<Vec<String>>,
    /// d tag
    #[arg(short, long, action = clap::ArgAction::Append)]
    d: Option<Vec<String>>,
    /// Since
    #[arg(short, long, action = clap::ArgAction::Append)]
    since: Option<u64>,
    /// Until
    #[arg(short, long, action = clap::ArgAction::Append)]
    until: Option<u64>,
    /// Limit
    #[arg(short, long, action = clap::ArgAction::Append)]
    limit: Option<usize>,
    /// Output
    #[arg(short, long)]
    output: Option<String>,
    /// Timeout in seconds
    #[arg(long)]
    timeout: Option<u64>,
}

pub fn list_events(relays: Vec<String>, sub_command_args: &ListEventsSubCommand) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let client = create_client(&Keys::generate(), relays, 0)?;

    let ids: Vec<String> = sub_command_args.ids.clone().unwrap_or(Vec::new());

    let authors: Vec<String> = sub_command_args.authors.clone().unwrap_or(Vec::new());

    // Convert kind number to Kind struct
    let kinds: Vec<Kind> = sub_command_args
        .kinds
        .clone()
        .unwrap_or(Vec::new())
        .into_iter()
        .map(Kind::from)
        .collect();

    // Convert event id string to EventId struct
    let events: Vec<EventId> = sub_command_args
        .e
        .clone()
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|e| {
            if e.starts_with("note1") {
                EventId::from_bech32(e.as_str()).expect("Invalid event id")
            } else {
                EventId::from_str(e.as_str()).expect("Invalid event id")
            }
        })
        .collect();

    // Convert pubkey strings to XOnlyPublicKey struct
    let pubkeys: Vec<XOnlyPublicKey> = sub_command_args
        .p
        .clone()
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|p| {
            Keys::from_pk_str(p.as_str())
                .expect("Invalid public key")
                .public_key()
        })
        .collect();

    let timeout = sub_command_args.timeout.map(|t| Duration::from_secs(t));

    let identifiers = sub_command_args.d.clone().unwrap_or(Vec::new());

    let events: Vec<Event> = client.get_events_of(
        vec![Filter {
            ids: ids,
            authors: authors,
            kinds,
            events,
            pubkeys,
            hashtags: Vec::new(),
            references: Vec::new(),
            search: None,
            since: sub_command_args.since.map(Timestamp::from),
            until: sub_command_args.until.map(Timestamp::from),
            limit: sub_command_args.limit,
            identifiers,
            generic_tags: HashMap::new(),
        }],
        timeout,
    )?;

    if let Some(output) = &sub_command_args.output {
        let file = std::fs::File::create(output)?;
        serde_json::to_writer_pretty(file, &events)?;
        println!("Wrote {} event(s) to {}", events.len(), output);
    } else {
        println!("{}", serde_json::to_string_pretty(&events)?)
    }

    Ok(())
}
