{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec
      {
        packages.amethyst = naersk-lib.buildPackage {
          pname = "Amethyst";
          root = ./.;
        };

        packages.default = packages.amethyst;

        apps.apod = utils.lib.mkApp {
          drv = packages.amethyst;
        };

        apps.default = apps.amethyst;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            cargo-audit
            rustfmt
            clippy
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        formatter = pkgs.alejandra;
      });
}
