use nostr_rust::req::ReqFilter;
use clap::{Args};

use ::nostr_tool::utils::create_client;

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
    kinds: Option<Vec<u16>>,
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
    limit: Option<u64>,
}

pub fn list_events(
    relays: Vec<String>,
    sub_command_args: &ListEventsSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let client = create_client(relays);

    let result = client
        .lock()
        .unwrap()
        .get_events_of(vec![ReqFilter {
            ids: sub_command_args.ids.clone(),
            authors: sub_command_args.authors.clone(),
            kinds: sub_command_args.kinds.clone(),
            e: sub_command_args.e.clone(),
            p: sub_command_args.p.clone(),
            since: sub_command_args.since,
            until: sub_command_args.until,
            limit: sub_command_args.limit,
        }]);
    match result {
        Ok(events) => {
            for (i, event) in events.iter().enumerate() {
                println!("{}: {:#?}", i, event);
            }
        }
        Err(e) => eprintln!("{}", e)
    }
}