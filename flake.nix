{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    fenix,
  }:
    utils.lib.eachDefaultSystem (system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        toolchain = with fenix.packages."${system}";
          combine [
            minimal.rustc
            minimal.cargo
            targets.x86_64-pc-windows-gnu.latest.rust-std
            targets.x86_64-unknown-linux-gnu.latest.rust-std
          ];
        naersk-lib = naersk.lib."${system}".override {
          cargo = toolchain;
          rustc = toolchain;
        };
      in rec
      {
        packages.amethyst = naersk-lib.buildPackage {
          pname = "Amethyst";
          root = ./.;
        };

        packages.amethyst-win = naersk-lib.buildPackage {
          pname = "Amethyst";
          root = ./.;
          strictDeps = true;
          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];
          nativeBuildInputs = with pkgs; [
            (
              if system == "x86_64-linux"
              then wineWowPackages.stable
              else hello
            )
          ];
          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = pkgs.writeScript "wine-wrapper" ''
            export WINEPREFIX="$(mktemp -d)"
            exec wine64 $@
          '';
          doCheck = true;
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

            openssl
            sqlite
            pkg-config
          ];
        };

        formatter = pkgs.alejandra;
      });
}
