use std::str::FromStr;

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
    /// p tag
    #[arg(short, long, action = clap::ArgAction::Append)]
    e: Option<Vec<String>>,
    /// p tag
    #[arg(short, long, action = clap::ArgAction::Append)]
    p: Option<Vec<String>>,
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
}

pub fn list_events(relays: Vec<String>, sub_command_args: &ListEventsSubCommand) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let client = create_client(&Keys::generate(), relays, 0)?;

    let authors: Option<Vec<XOnlyPublicKey>> = sub_command_args.authors.as_ref().map(|auths| {
        auths
            .iter()
            .map(|a| XOnlyPublicKey::from_str(a.as_str()).expect("Invalid public key"))
            .collect()
    });

    let kinds: Option<Vec<Kind>> = sub_command_args
        .kinds
        .as_ref()
        .map(|kinds| kinds.iter().map(|k| Kind::from(*k)).collect());

    let events: Option<Vec<EventId>> = sub_command_args.e.as_ref().map(|events| {
        events
            .iter()
            .map(|e| EventId::from_hex(e.as_str()).expect("Invalid event id"))
            .collect()
    });

    let pubkeys: Option<Vec<XOnlyPublicKey>> = sub_command_args.p.as_ref().map(|pubs| {
        pubs.iter()
            .map(|p| XOnlyPublicKey::from_str(p.as_str()).expect("Invalid public key"))
            .collect()
    });

    let events: Vec<Event> = client.get_events_of(
        vec![Filter {
            ids: sub_command_args.ids.clone(),
            authors,
            kinds,
            events,
            pubkeys,
            hashtags: None,
            references: None,
            search: None,
            since: sub_command_args.since.map(Timestamp::from),
            until: sub_command_args.until.map(Timestamp::from),
            limit: sub_command_args.limit,
            custom: Map::new(),
        }],
        None,
    )?;

    if let Some(output) = &sub_command_args.output {
        let file = std::fs::File::create(output).unwrap();
        serde_json::to_writer_pretty(file, &events).unwrap();
        println!("Wrote {} event(s) to {}", events.len(), output);
    } else {
        for (i, event) in events.iter().enumerate() {
            if let Ok(e) = serde_json::to_string_pretty(event) {
                println!("{i}: {e:#}")
            }
        }
    }

    Ok(())
}
