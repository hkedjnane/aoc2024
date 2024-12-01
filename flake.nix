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
            cargo
            rustc
            python311Packages.requests

            git-crypt
          ];

          YEAR = 2024;
          SESSION = builtins.readFile ./.session;

          shellHook = ''
          export PATH=$PATH:$PWD/scripts
          export ROOT=$PWD
          '';
        };
      }
    );
}
