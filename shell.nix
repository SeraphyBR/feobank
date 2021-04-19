{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [ stdenv ncurses ];
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [
        sqlite
        sqlx-cli
        rustc
        cargo
        cargo-edit

        # Use steam-run, for a fhs enviroment, allow vscode code-lldb debugger to run
        # $ steam-run code .
        (steam.override {
          extraPkgs = pkgs: [ ncurses zlib ];
          nativeOnly = true;
        }).run
    ];
    shellHook = ''
    if [ ! -e "feobank.db" ]; then
        sqlx database create
        sqlx migrate run
    fi
    '';
}
