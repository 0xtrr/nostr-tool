# Changelog

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
