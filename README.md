# Nostr-tools

A simple CLI tool to send nostr events.

Currently, this is more of a POC on the use of the [nostr_rust](https://github.com/0xtlt/nostr_rust) library in a CLI
application.

## Install

Clone the repo and run the following command in the repo folder

```shell
cargo install --path .
```

## Examples

### Update metadata
```shell
nostr-tool -r "wss://nostr.oxtr.dev" update-metadata -n "Alice" -a "Who the fuck is Alice?" -p "https://upload.wikimedia.org/wikipedia/en/2/2b/New_world-living_next_door_to_alice.JPG"
```

### Create a new note with a new identity

```shell
nostr-tool -r "wss://nostr.oxtr.dev" text-note -c "Hello World"
```

### Create a new note with an existing private key

```shell
nostr-tool -r "wss://nostr.oxtr.dev" -p "PRIVATE-KEY" text-note -c "Hello World"
```

### Create a new note with an existing private key as a reply to another note

```shell
nostr-tool -r "wss://nostr.oxtr.dev" -p "PRIVATE-KEY" text-note -c "Hello World" --etag "EVENT-ID TO REPLY TO" --ptag "PUBKEY YOU'RE REPLYING TO"
```