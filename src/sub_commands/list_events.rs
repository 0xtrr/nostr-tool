use std::{str::FromStr, time::Duration};

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
    #[arg(long, action = clap::ArgAction::Append)]
    etag: Option<Vec<String>>,
    /// p tag
    #[arg(long, action = clap::ArgAction::Append)]
    ptag: Option<Vec<String>>,
    /// d tag
    #[arg(long, action = clap::ArgAction::Append)]
    dtag: Option<Vec<String>>,
    /// a tag
    #[arg(long, action = clap::ArgAction::Append)]
    atag: Option<Vec<String>>,
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

pub async fn list_events(relays: Vec<String>, sub_command_args: &ListEventsSubCommand) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let client = create_client(&Keys::generate(), relays, 0).await?;
    let mut filter = Filter::new();

    // Handle event ids
    if sub_command_args.ids.is_some() {
        let ids: Vec<EventId> = sub_command_args.ids.clone()
            .unwrap_or(Vec::new())
            .iter()
            .map(|id| EventId::from_str(id).unwrap())
            .collect();
        filter = filter.ids(ids);
    }

    // Handle author public keys
    if sub_command_args.authors.is_some() {
        let authors: Vec<PublicKey> = sub_command_args.authors.clone()
            .unwrap_or(Vec::new())
            .iter()
            .map(|author_pubkey| PublicKey::from_str(author_pubkey).unwrap())
            .collect();
        filter = filter.authors(authors);
    }

    // Handle kind numbers
    if sub_command_args.kinds.is_some() {
        // Convert kind number to Kind struct
        let kinds: Vec<Kind> = sub_command_args
            .kinds
            .clone()
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|x| x as u16)
            .map(Kind::from)
            .collect();
        filter = filter.kinds(kinds);
    }

    // Handle e-tags
    if sub_command_args.etag.is_some() {
        // Convert event id string to EventId struct
        let events: Vec<EventId> = sub_command_args
            .etag
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
        filter = filter.events(events);
    }

    // Handle p-tags
    if sub_command_args.ptag.is_some() {
        // Convert pubkey strings to XOnlyPublicKey struct
        let pubkeys: Vec<PublicKey> = sub_command_args
            .ptag
            .clone()
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|p| {
                PublicKey::from_str(p.as_str())
                    .expect("Invalid public key")
            })
            .collect();
        filter = filter.pubkeys(pubkeys);
    }

    // Handle d-tags
    if sub_command_args.dtag.is_some() {
        filter = filter.identifiers(sub_command_args.dtag.clone().unwrap_or(Vec::new()));
    }

    if sub_command_args.since.is_some() {
        filter = filter.since(sub_command_args.since.map(Timestamp::from).unwrap())
    }

    if sub_command_args.until.is_some() {
        filter = filter.until(sub_command_args.until.map(Timestamp::from).unwrap())
    }

    if sub_command_args.limit.is_some() {
        filter = filter.limit(sub_command_args.limit.unwrap())
    }

    let timeout = sub_command_args.timeout.map(Duration::from_secs);

    let events: Vec<Event> = client.get_events_of(vec![filter], timeout).await?;

    if let Some(output) = &sub_command_args.output {
        let file = std::fs::File::create(output)?;
        serde_json::to_writer_pretty(file, &events)?;
        println!("Wrote {} event(s) to {}", events.len(), output);
    } else {
        println!("{}", serde_json::to_string_pretty(&events)?)
    }

    Ok(())
}
