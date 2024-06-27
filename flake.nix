{
  description = "Nostr-tool flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        inputs = [
          rust
          pkgs.rust-analyzer
          pkgs.openssl
          pkgs.zlib
          pkgs.gcc
          pkgs.pkg-config
          pkgs.clang
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "nostr-tool";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = inputs;
        };
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          packages = inputs;
          shellHook = ''
            	    export LIBCLANG_PATH=${pkgs.libclang.lib}/lib/
                        export LD_LIBRARY_PATH=${pkgs.openssl}/lib:$LD_LIBRARY_PATH
          '';
        };
      }
    );
}
