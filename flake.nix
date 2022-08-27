{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages = rec {
          scieldas = naersk-lib.buildPackage {
            root = ./.;
            buildInputs = with pkgs; [ pkg-config openssl ];
            doCheck = true;
          };

          docker =
            let
              inherit (self.packages.${system}) scieldas;
            in
            pkgs.dockerTools.buildLayeredImage {
              name = "scieldas";
              contents = [ scieldas ];
              config = {
                Env = [
                  "ROCKET_LOG_LEVEL=debug"
                  "ROCKET_ADDRESS=0.0.0.0"
                ];
                Cmd = [
                  "${scieldas}/bin/scieldas"
                ];
                ExposedPorts = {
                  "8000/tcp" = { };
                };
              };
            };

          default = self.packages.${system}.scieldas;
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy dhall openssl pkg-config ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
