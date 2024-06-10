use std::{process::exit, str::FromStr, time::Duration};

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

#[derive(Args)]
pub struct AwardBadgeSubCommand {
    /// Badge definition event id
    #[arg(short, long)]
    badge_event_id: String,
    /// Awarded pubkeys
    #[arg(short, long, action = clap::ArgAction::Append)]
    ptag: Vec<String>,
}

pub async fn award_badge(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &AwardBadgeSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = parse_private_key(private_key, true).await?;
    let client: Client = create_client(&keys, relays, difficulty_target).await?;

    let event_id: EventId = EventId::from_str(sub_command_args.badge_event_id.as_str())?;
    let badge_definition_query = client
        .get_events_of(
            vec![Filter::new().id(event_id)],
            Some(Duration::from_secs(10)),
        )
        .await?;

    if badge_definition_query.len() != 1 {
        eprintln!("Expected one event, got {}", badge_definition_query.len());
        exit(1)
    };

    let badge_definition_event = badge_definition_query.get(0).unwrap();
    // Verify that this event is a badge definition event
    if badge_definition_event.kind != Kind::BadgeDefinition {
        eprintln!(
            "Unexpected badge definition event. Exepected event of kind {} but got {}",
            Kind::BadgeDefinition.as_u32(),
            badge_definition_event.kind.as_u32()
        );
        exit(1)
    }

    // Verify that the user trying to award the badge is actually the author of the badge definition
    if badge_definition_event.pubkey != keys.public_key() {
        eprint!("Incorrect private key. Only the private key used for issuing the badge definition can award it to other public keys");
        exit(1)
    }

    let awarded_pubkeys: Vec<Tag> = sub_command_args
        .ptag
        .iter()
        .map(|pubkey_string| {
            Tag::public_key(
                public_key::PublicKey::from_str(pubkey_string).expect("Unable to parse public key"),
            )
        })
        .collect();

    let event = EventBuilder::award_badge(badge_definition_event, awarded_pubkeys)?
        .to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event).await?;

    println!("Published badge award event with id:");
    println!("Hex: {}", event_id.to_hex());
    println!("Bech32: {}", event_id.to_bech32()?);

    Ok(())
}
