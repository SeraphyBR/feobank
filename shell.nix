{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [
        sqlite
        sqlx-cli
        rustc
        cargo
        cargo-edit
    ];
    shellHook = ''
    if [ ! -e "feobank.db" ]; then
        sqlx database create
        sqlx migrate run
    fi
    '';
}