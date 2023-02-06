use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;
use serde::Deserialize;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct PublishContactListCsvSubCommand {
    /// Path to CSV file. CSV file should be have the following format:
    /// pubkey,relay_url,petname. See example in resources/contact_list.csv
    #[arg(short, long)]
    filepath: String,
}

// nostr_rust ContactListTag struct does not derive "Deserialize", therefore we need this custom implementation
#[derive(Debug, Clone, Deserialize)]
pub struct ContactListTag {
    /// 32-bytes hex key - the public key of the contact
    pub pubkey: String,
    /// main relay URL
    pub relay: Option<String>,
    /// Petname
    pub petname: Option<String>,
}

pub fn publish_contact_list_from_csv_file(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &PublishContactListCsvSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(private_key)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    let mut rdr = csv::Reader::from_path(&sub_command_args.filepath)?;
    let mut contacts: Vec<Contact> = vec![];
    for result in rdr.deserialize() {
        let tag: ContactListTag = result?;
        let clt = Contact {
            pk: XOnlyPublicKey::from_str(&tag.pubkey)?,
            relay_url: tag.relay,
            alias: tag.petname,
        };
        contacts.push(clt);
    }

    client.set_contact_list(contacts)?;
    println!("Contact list imported!");
    Ok(())
}
