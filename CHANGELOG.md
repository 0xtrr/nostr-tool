# Changelog

## 0.5.0 - TBD

### Added
- Support for NIP-58 Badges by [@0xtrr](https://github.com/0xtrr)
- Add support for a tags in list events command by [@0xtrr](https://github.com/0xtrr)
- Add support for NIP-315 user statuses by [@0xtrr](https://github.com/0xtrr)

### Changed
- Upgrade nostr-sdk from 0.22.0 to 0.23.0 by [@0xtrr](https://github.com/0xtrr)
- 

### Fixed

## 0.4.0 - 2023-06-09

### Added
- Save output from list-events to file by [@thesimplekid](https://github.com/thesimplekid)
- Broadcast events from json file by [@thesimplekid](https://github.com/thesimplekid)
- Add option to print keys as hex instead of bech32 by [@thesimplekid](https://github.com/thesimplekid)
- Add LUD-16 to update metadata command by [@w3irdrobot](https://github.com/w3irdrobot)
- Add support for creating NIP-57 events (Zaps) by [@0xtrr](https://github.com/0xtrr)
- Add support for custom events with arbitrary kind, content and tags by [@0xtrr](https://github.com/0xtrr)
- Add timeout argument for list-events command by [@thesimplekid](https://github.com/thesimplekid)
- Add bech32 support pubkeys and events for list-events command by [@thesimplekid](https://github.com/thesimplekid)


### Changed
- Upgrade nostr-sdk from 0.18 to 0.20 by [@thesimplekid](https://github.com/thesimplekid)
- Upgrade nostr-sdk from 0.20 to 0.21 by [@0xtrr](https://github.com/0xtrr)
- Upgrade nostr-sdk from 0.21 to 0.22 by [@0xtrr](https://github.com/0xtrr)
- Remove bitcoin dependency by [@thesimplekid](https://github.com/thesimplekid)

### Fixed
- Print events as valid json in list-events command by [@thesimplekid](https://github.com/thesimplekid)

## 0.3.0 - 2023-02-20

### Added
- NIP-28 support.
- Add expiration tag to text-note by [@thesimplekid](https://github.com/thesimplekid).
- Add Dockerfile by [@bijeebuss](https://github.com/bijeebuss).
- Add .devcontainer by [@bijeebuss](https://github.com/bijeebuss).
- Add encoding/decoding of nprofile strings
- Add NIP-14 (subject tags) support to text notes
- Add support for encoding/decoding bech32 encoded nchannel ids

### Changed
- Upgrade dependency Clap from 4.0.22 to 4.1.6.
- Big rewrite by [@yukibtc](https://github.com/yukibtc) to replace nostr_rust with [nostr-sdk](https://github.com/rust-nostr/nostr).
- Print nchannel id when creating new public channel

### Fixed
- Update typo in examples in Readme by [@gourcetools](https://github.com/gourcetools).
- parse_key function misbehaved after nostr-sdk refactoring.
- Refactor/code cleanup in "list events" code by [@thesimplekid](https://github.com/thesimplekid).
- Pretty print events in "list events" command output by [@thesimplekid](https://github.com/thesimplekid).


## 0.2.0 - 2023-01-08

### Added
- Support for bech32 encoded keys and notes in commands.
- Add command for generating a new keypair.
- Add command for key/note id conversion between bech32 and hex encodings.
- Add crated badge to readme.
- Add MIT licence. Idc, just use it to whatever you want as long as I'm not liable for it.

### Changed
- Refactored codebase to increase readability and isolate different concerns.
- 

## 0.1.0
- Edit: Upgrade `secp256k1` from `0.24` to `0.25`.
- Add: `update-metadata` command.
- Add: `text-note` command.
- Add: `recommend-server` command.
- Add: `publish-contact-list-csv` command.
- Add: `send-direct-message` command.
- Add: `delete-event` command.
- Add: `react` command.
- Add: `list-events` command.
