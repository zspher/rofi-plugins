{
  description = "example rust program";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
                self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          if builtins.pathExists ./rust-toolchain.toml then
            rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain then
            rust.fromRustupToolchainFile ./rust-toolchain
          else
            rust.stable.latest.default.override {
              extensions = [
                "rust-src"
                "rustfmt"
              ];
            };
      };
      devShells = perSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              cargo-deny
              cargo-edit
              cargo-watch
              rust-analyzer

              rofi-wayland-unwrapped
              pkg-config
              glib
              cairo
              pango
              just
            ];

            shellHook = ''
              # Required by rust-analyzer
              export RUST_SRC_PATH=${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            '';
          };
        }
      );
      packages = perSystem (
        { pkgs }:
        let
          lockFile = ./Cargo.lock;
        in
        rec {
          rofi-websearch = pkgs.callPackage ./nix/plugin.nix {
            inherit lockFile;
            name = "rofi-websearch";
          };
          rofi-with-plug = (
            pkgs.rofi-wayland.override {
              plugins = [
                rofi-websearch
              ];
            }
          );
        }
      );
    };
}
