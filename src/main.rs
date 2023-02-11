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
        Commands::Vanity(sub_command_args) => sub_commands::vanity::vanity(sub_command_args),
        Commands::CreatePublicChannel(sub_command_args) => sub_commands::create_public_channel::create_public_channel(
            args.private_key,
            args.relays,
            args.difficulty_target,
            sub_command_args,
        )
    }
}
