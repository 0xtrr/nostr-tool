use std::str::FromStr;
use std::time::Duration;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, parse_private_key};

#[derive(Args)]
pub struct ProfileBadgesSubCommand {
    /// Badge definition event id
    #[arg(short, long, action = clap::ArgAction::Append)]
    badge_id: Vec<String>,
    /// Badge award event id
    #[arg(short, long, action = clap::ArgAction::Append)]
    award_id: Vec<String>,
}

pub async fn set_profile_badges(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &ProfileBadgesSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = parse_private_key(private_key, true).await?;
    let client: Client = create_client(&keys, relays, difficulty_target).await?;

    let badge_definition_event_ids: Vec<EventId> = sub_command_args
        .badge_id
        .iter()
        .map(|badge_id| EventId::from_str(badge_id).unwrap())
        .collect();
    let badge_definition_filter = Filter::new()
        .ids(badge_definition_event_ids)
        .kind(Kind::BadgeDefinition);
    let badge_defintion_events = client
        .get_events_of(vec![badge_definition_filter], Some(Duration::from_secs(10)))
        .await
        .unwrap();

    let award_event_ids: Vec<EventId> = sub_command_args
        .award_id
        .iter()
        .map(|award_event_id| EventId::from_str(award_event_id).unwrap())
        .collect();
    let badge_award_filter = Filter::new().ids(award_event_ids).kind(Kind::BadgeAward);
    let badge_award_events = client
        .get_events_of(vec![badge_award_filter], Some(Duration::from_secs(10)))
        .await
        .unwrap();

    let event = EventBuilder::profile_badges(
        badge_defintion_events,
        badge_award_events,
        &keys.public_key(),
    )?
    .to_pow_event(&keys, difficulty_target)?;

    // Publish event
    let event_id = client.send_event(event).await?;
    println!("Published profile badges event with id:");
    println!("Hex: {}", event_id.to_hex());
    println!("Bech32: {}", event_id.to_bech32()?);

    Ok(())
}
