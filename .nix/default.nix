{ pkgs ? import <nixpkgs> {} }:

let
  rustPkgs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ];  
  pkgDeps = with pkgs; [
    openssl
    pkgconfig
    sqlite
  ];
in
  pkgs.mkShell {
    nativeBuildInputs = rustPkgs ++ pkgDeps; 
    RUST_BACKTRACE = 1;
  }
