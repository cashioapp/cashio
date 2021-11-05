{ pkgs }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    (import ./ci.nix { inherit pkgs; })
    rustup
    cargo-deps
    gh
    spl-token-cli
  ];
}
