{
  description = "Privacy-first domain management CLI for Njalla";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    {
      nixosModules.default = import ./module.nix;
    } //
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        # Common build inputs
        buildInputs = with pkgs; [
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "njalla-cli";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            inherit buildInputs nativeBuildInputs;

            meta = with pkgs.lib; {
              description = "Privacy-first domain management CLI for Njalla";
              homepage = "https://github.com/gudnuf/njalla-cli";
              license = licenses.mit;
              mainProgram = "njalla";
            };
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ [
            rustToolchain
            pkgs.cargo-watch
            pkgs.cargo-edit
          ];

          inherit nativeBuildInputs;

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "njalla-cli dev shell"
            echo "  cargo build    - Build debug"
            echo "  cargo test     - Run tests"
            echo "  cargo clippy   - Lint"
            echo "  nix build      - Build release"
            export NJALLA_API_TOKEN="''${NJALLA_API_TOKEN:-}"
          '';
        };

        # Formatter for `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
