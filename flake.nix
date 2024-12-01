{
  description = "Rust devShell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rustup
            python311Packages.requests

            git-crypt
          ];

          YEAR = 2024;
          SESSION = builtins.readFile ./.session;

          shellHook = ''
          rustup default stable
          rustup update
          rustup component add cargo clippy rustc rust-analyzer rustfmt
          export PATH=$PATH:$PWD/scripts
          export ROOT=$PWD
          '';
        };
      }
    );
}
