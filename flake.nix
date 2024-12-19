{
  description = "zenoh";

  inputs = {
    crate2nix.url    = "github:nix-community/crate2nix";
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = inputs@{ flake-utils, ... }:
    flake-utils.lib.meld inputs [
      # ./nix/modules
      ./nix/packages/zenoh.nix
      ./nix/overlay.nix
      ./nix/shell.nix
    ];
}
