{ pkgs }:
pkgs.stdenvNoCC.mkDerivation {
  name = "devenv";
  buildInputs = with pkgs; [
    (import ./ci.nix { inherit pkgs; })
    rustup
    cargo-deps
    gh
    spl-token-cli
  ];
}
