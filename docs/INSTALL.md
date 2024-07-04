Clone the repo and run the following command in the repo folder. You must have Rust installed to compile this.

### Install from crates.io
```shell
cargo install nostr-tool
```

### Build from source
```shell
cargo build --release
```

### Build with Docker locally
```shell
docker build -t nostr-tool .
```
### Build with Docker from DockerHub
```shell
docker pull 0xtr/nostr-tool:0.3.0
```

### Build with Nix locally
```shell
nix develop 
cargo build --release
```