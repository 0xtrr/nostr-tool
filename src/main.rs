use clap::{Parser, Subcommand};
use nostr_sdk::Result;

mod sub_commands;
mod utils;

/// Simple CLI application to interact with nostr
#[derive(Parser)]
#[command(name = "nostr-tool")]
#[command(author = "0xtr. <oxtrr@protonmail.com")]
#[command(version = "0.1")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// User private key
    #[arg(short, long)]
    private_key: Option<String>,
    /// Relay to connect to
    #[arg(short, long, action = clap::ArgAction::Append)]
    relays: Vec<String>,
    /// Proof of work difficulty target
    #[arg(short, long, action = clap::ArgAction::Append, default_value_t = 0)]
    difficulty_target: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Update metadata
    UpdateMetadata(sub_commands::update_metadata::UpdateMetadataSubCommand),
    /// Send text note
    TextNote(sub_commands::text_note::TextNoteSubCommand),
    /// Recommend a relay
    RecommendRelay(sub_commands::recommend_relay::RecommendRelaySubCommand),
    /// Publish contacts from a CSV file
    PublishContactListCsv(sub_commands::publish_contactlist_csv::PublishContactListCsvSubCommand),
    /// Send a direct message
    SendDirectMessage(sub_commands::dm::SendDirectMessageSubCommand),
    /// Delete an event
    DeleteEvent(sub_commands::delete_event::DeleteEventSubCommand),
    /// React to an event
    React(sub_commands::react::ReactionSubCommand),
    /// Get all events
    ListEvents(sub_commands::list_events::ListEventsSubCommand),
    /// Generate a new keypair
    GenerateKeypair(sub_commands::generate_keypair::GenerateKeypairSubCommand),
    /// Convert key from bech32 to hex or hex to bech32
    ConvertKey(sub_commands::convert_key::ConvertKeySubCommand),
    /// Vanity public key mining
    Vanity(sub_commands::vanity::VanitySubCommand),
    /// Create a new public channel
    CreatePublicChannel(sub_commands::create_public_channel::CreatePublicChannelSubCommand),
    /// Update channel metadata
    SetChannelMetadata(sub_commands::set_channel_metadata::SetChannelMetadataSubCommand),
    /// Send a message to a public channel
    SendChannelMessage(sub_commands::send_channel_message::SendChannelMessageSubCommand),
    /// Hide a message in a public chat room
    HidePublicChannelMessage(
        sub_commands::hide_public_channel_message::HidePublicChannelMessageSubCommand,
    ),
    /// Mute a public key
    MutePublicKey(sub_commands::mute_publickey::MutePublickeySubCommand),
    /// Encode/Decode a nprofile string (bech32 encoded)
    Nprofile(sub_commands::nprofile::NprofileSubCommand),
    /// Broadcast events from file
    BroadcastEvents(sub_commands::broadcast_events::BroadcastEventsSubCommand),
    /// Create a zap request. Currently just prints the json to console, you need to send the HTTP request yourself.
    CreateZapRequest(sub_commands::zap_request::CreateZapRequestCommand),
    /// Send a zap receipt note.
    CreateZapReceipt(sub_commands::zap_reciept::SendZapSubCommand),
    /// Create a new badge
    CreateBadge(sub_commands::create_badge::CreateBadgeSubCommand),
    /// Publish award badge event
    AwardBadge(sub_commands::award_badge::AwardBadgeSubCommand),
    /// Set profile badges
    ProfileBadges(sub_commands::profile_badges::ProfileBadgesSubCommand),
    /// Create custom event
    CustomEvent(sub_commands::custom_event::CustomEventCommand),
    /// Create a user status event
    SetUserStatus(sub_commands::user_status::UserStatusSubCommand),
}

fn main() -> Result<()> {
    // Parse input
    let args: Cli = Cli::parse();

    // Post event
    match &args.command {
        Commands::UpdateMetadata(sub_command_args) => {
            sub_commands::update_metadata::update_metadata(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::TextNote(sub_command_args) => sub_commands::text_note::broadcast_textnote(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::RecommendRelay(sub_command_args) => {
            sub_commands::recommend_relay::recommend_relay(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::PublishContactListCsv(sub_command_args) => {
            sub_commands::publish_contactlist_csv::publish_contact_list_from_csv_file(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::SendDirectMessage(sub_command_args) => sub_commands::dm::send(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::DeleteEvent(sub_command_args) => sub_commands::delete_event::delete(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::React(sub_command_args) => sub_commands::react::react_to_event(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::ListEvents(sub_command_args) => {
            sub_commands::list_events::list_events(args.relays, sub_command_args)
        }
        Commands::GenerateKeypair(sub_command_args) => {
            sub_commands::generate_keypair::get_new_keypair(sub_command_args)
        }
        Commands::ConvertKey(sub_command_args) => {
            sub_commands::convert_key::convert_key(sub_command_args)
        }
        Commands::Nprofile(sub_command_args) => sub_commands::nprofile::nprofile(sub_command_args),
        Commands::Vanity(sub_command_args) => sub_commands::vanity::vanity(sub_command_args),
        Commands::CreatePublicChannel(sub_command_args) => {
            sub_commands::create_public_channel::create_public_channel(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::SetChannelMetadata(sub_command_args) => {
            sub_commands::set_channel_metadata::set_channel_metadata(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::SendChannelMessage(sub_command_args) => {
            sub_commands::send_channel_message::send_channel_message(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::HidePublicChannelMessage(sub_command_args) => {
            sub_commands::hide_public_channel_message::hide_public_channel_message(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::MutePublicKey(sub_command_args) => sub_commands::mute_publickey::mute_publickey(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::BroadcastEvents(sub_command_args) => {
            sub_commands::broadcast_events::broadcast_events(args.relays, sub_command_args)
        }
        Commands::CreateZapRequest(sub_command_args) => {
            sub_commands::zap_request::create_zap_request(
                args.private_key,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::CreateZapReceipt(sub_command_args) => {
            sub_commands::zap_reciept::send_zap_receipt(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::CreateBadge(sub_command_args) => sub_commands::create_badge::create_badge(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::AwardBadge(sub_command_args) => sub_commands::award_badge::award_badge(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::ProfileBadges(sub_command_args) => {
            sub_commands::profile_badges::set_profile_badges(
                args.private_key,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
        }
        Commands::CustomEvent(sub_command_args) => sub_commands::custom_event::create_custom_event(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
        Commands::SetUserStatus(sub_command_args) => sub_commands::user_status::set_user_status(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        ),
    }
}
