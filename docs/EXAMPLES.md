## Examples

### Update metadata
```shell
nostr-tool -r wss://nostr.oxtr.dev update-metadata -n "Alice" -a "Who the fuck is Alice?" -p "https://upload.wikimedia.org/wikipedia/en/2/2b/New_world-living_next_door_to_alice.JPG"
```

### Create a new note with a new identity

```shell
nostr-tool -r wss://nostr.oxtr.dev text-note -c "Hello World"
```

### Create a new note with an existing private key

```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} text-note -c "Hello World"
```

### Create a new note with an existing private key as a reply to another note

```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} text-note -c "Hello World" --etag {EVENT-ID_TO_REPLY_TO} --ptag {PUBKEY_YOU_ARE_REPLYING_TO}
```

### Import contacts/followers from a CSV file

```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} publish-contact-list-csv -f {PATH_TO_CSV_FILE}
```

The CSV file should have the following format
```csv
pubkey,relay,petname
b2d670de53b27691c0c3400225b65c35a26d06093bcc41f48ffc71e0907f9d4a,"wss://nostr.oxtr.dev",""
32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245,"wss://relay.damus.io",""
```

### Send a direct message

```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} send-direct-message --receiver {RECIPIENT_PUBKEY} --message "Hello World"
```

### Delete an event

```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} delete-event -e {EVENT_ID} -r "The reason for deleting the event"
```

### Delete a profile

Just events:
```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} delete-profile --events-only --kinds 1
```

Delete metadata profile:
```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} delete-profile
```

### React to an event

```shell
nostr-tool -r wss://nostr.oxtr.dev -p {PRIVATE_KEY} react -e {EVENT_ID} -a {EVENT_AUTHOR_PUBKEY} -r "👍"
```

### Run with docker
```shell
docker run nostr-tool nostr-tool -r wss://nostr.oxtr.dev text-note -c "Hello World"
```
