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
}

pub fn list_events(relays: Vec<String>, sub_command_args: &ListEventsSubCommand) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let client = create_client(&Keys::generate(), relays, 0);

    let authors: Option<Vec<XOnlyPublicKey>> = match &sub_command_args.authors {
        None => None,
        Some(auths) => Some(
            auths
                .iter()
                .map(|a| XOnlyPublicKey::from_str(a.as_str()).expect("Invalid public key"))
                .collect(),
        ),
    };

    let kinds: Option<Vec<Kind>> = match &sub_command_args.kinds {
        None => None,
        Some(kinds) => Some(kinds.iter().map(|k| Kind::from(*k)).collect()),
    };

    let events: Option<Vec<EventId>> = match &sub_command_args.e {
        None => None,
        Some(events) => Some(
            events
                .iter()
                .map(|e| EventId::from_hex(e.as_str()).expect("Invalid event id"))
                .collect(),
        ),
    };

    let pubkeys: Option<Vec<XOnlyPublicKey>> = match &sub_command_args.p {
        None => None,
        Some(pubs) => Some(
            pubs.iter()
                .map(|p| XOnlyPublicKey::from_str(p.as_str()).expect("Invalid public key"))
                .collect(),
        ),
    };

    let result = client.get_events_of(vec![SubscriptionFilter {
        ids: sub_command_args.ids.clone(),
        authors: authors,
        kinds: kinds,
        events: events,
        pubkeys: pubkeys,
        hashtags: None,
        references: None,
        search: None,
        since: sub_command_args.since.map(Timestamp::from),
        until: sub_command_args.until.map(Timestamp::from),
        limit: sub_command_args.limit,
    }]);
    match result {
        Ok(events) => {
            for (i, event) in events.iter().enumerate() {
                println!("{i}: {event:#?}");
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}
