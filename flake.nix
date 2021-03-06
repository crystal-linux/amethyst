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
      packages.amethyst = naersk-lib.buildPackage {
        pname = "ame";
        root = ./.;
        nativeBuildInputs = with pkgs; [
          openssl
          sqlite
          pkg-config
        ];
      };
      
      packages.default = packages.amethyst;

      apps.amethyst = utils.lib.mkApp {
        drv = packages.amethyst;
      };
      
      apps.default = apps.amethyst;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          cargo-audit
          clippy
          openssl
          sqlite
          pkg-config
        ];
      };
    });
}
