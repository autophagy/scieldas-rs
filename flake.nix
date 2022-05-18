{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/master";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=master";
  };

  outputs = { self, nixpkgs, cargo2nix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlay];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.60.0";
          packageFun = import ./Cargo.nix;
        };

      in rec {
        packages = {
          scieldas = (rustPkgs.workspace.scieldas {}).bin;
        };
        defaultPackage = packages.scieldas;

        # Nix develop
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo rustfmt clippy openssl ];
        };
      }
    );
}
