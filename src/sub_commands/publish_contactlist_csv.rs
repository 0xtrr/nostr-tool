use clap::{Args};
use serde::Deserialize;

use ::nostr_tool::utils::handle_identity;
use ::nostr_tool::utils::create_client;

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
    difficulty_target: u16,
    sub_command_args: &PublishContactListCsvSubCommand
) {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let identity = handle_identity(private_key);
    let client = create_client(relays);

    let mut rdr = csv::Reader::from_path(&sub_command_args.filepath).unwrap();
    let mut contacts: Vec<nostr_rust::nips::nip2::ContactListTag> = vec![];
    for result in rdr.deserialize() {
        let tag: ContactListTag = result.unwrap();
        let clt = nostr_rust::nips::nip2::ContactListTag {
            key: tag.pubkey,
            main_relay: tag.relay,
            surname: tag.petname,
        };
        contacts.push(clt);
    }
    let result = client
        .lock()
        .unwrap()
        .set_contact_list(&identity, contacts, difficulty_target);
    match result {
        Ok(_) => println!("Contact list imported!"),
        Err(e) => eprintln!("{}", e)
    }
}