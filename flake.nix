{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec 
    {
      packages.ame = naersk-lib.buildPackage {
        pname = "amethyst";
        root = ./.;
        nativeBuildInputs = with pkgs; [
          openssl
          sqlite
          pkg-config
        ];
      };
      defaultPackage = packages.ame;

      apps.ame = utils.lib.mkApp {
        drv = packages.amethyst;
      };
      defaultApp = apps.ame;

      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
        ];
      };
    });
}


