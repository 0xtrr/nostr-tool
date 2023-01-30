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

    let mut authors: Vec<XOnlyPublicKey> = Vec::new();
    if let Some(auths) = &sub_command_args.authors {
        for author in auths.iter() {
            authors.push(XOnlyPublicKey::from_str(author.as_str()).expect("Invalid public key"));
        }
    }

    let mut kinds: Vec<Kind> = Vec::new();
    if let Some(ks) = &sub_command_args.kinds {
        for kind in ks.iter() {
            kinds.push(Kind::from(*kind));
        }
    }

    let mut events: Vec<EventId> = Vec::new();
    if let Some(evns) = &sub_command_args.e {
        for event in evns.iter() {
            events.push(EventId::from_hex(event.as_str()).expect("Invalid event id"));
        }
    }

    let mut pubkeys: Vec<XOnlyPublicKey> = Vec::new();
    if let Some(pks) = &sub_command_args.p {
        for pubkey in pks.iter() {
            pubkeys.push(XOnlyPublicKey::from_str(pubkey.as_str()).expect("Invalid public key"));
        }
    }

    let result = client.get_events_of(vec![SubscriptionFilter {
        ids: sub_command_args.ids.clone(),
        authors: Some(authors),
        kinds: Some(kinds),
        events: Some(events),
        pubkeys: Some(pubkeys),
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
