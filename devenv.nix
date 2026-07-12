{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

let
  esp-config = pkgs.rustPlatform.buildRustPackage rec {
    pname = "esp-config";
    version = "0.7.0"; # change to the version you want

    src = pkgs.fetchCrate {
      inherit pname version;
      hash = "sha256-1vEdp6ln0B72xEOcd4Tci9tG3ij62IDm7Kh4HhB37Lc=";
    };

    cargoHash = "sha256-BP2AVHNkqNJ/LZtkQS4H5+x2H6YfqWu4cVMeir5Mkqs=";

    buildFeatures = [ "tui" ];
  };
  espsegs = pkgs.rustPlatform.buildRustPackage rec {
    pname = "espsegs";
    version = "0.1.0"; # change to the version you want

    src = pkgs.fetchFromGitHub {
      owner = "bjoernQ";
      repo = "espsegs";
      rev = "e20009ea369337a0240f7ab04e8d1b3753d88889";
      hash = "sha256-syEvu/qGB4XbXmP+hMgcGVyEiPwOci9seIwnH/aRPWg=";
    };

    cargoHash = "sha256-h64JI2WmybSCyNkP9C0aIJF1sgazYFDNlc+QZYvw74A=";

  };
in
{

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    probe-rs-tools
    minicom
    cargo-binutils
    esp-generate
    espflash
    esp-config
    espsegs
    cargo-deny
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    targets = [ "riscv32imc-unknown-none-elf" ];
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "llvm-tools"
    ];
  };

  scripts.r.exec = "cargo run";
  # See full reference at https://devenv.sh/reference/options/
}
