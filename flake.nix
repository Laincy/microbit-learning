{
  description = "Learning embedded Rust on Microbit V2";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};

      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src"];
        targets = ["thumbv7em-none-eabihf"];
      };
    in {
      devShells.default =
        pkgs.mkShell
        {
          name = "microbit-dev";
          buildInputs = with pkgs; [
            toolchain
            cargo-binutils
            probe-rs-tools
          ];
        };
    });
}
