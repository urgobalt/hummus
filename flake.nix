{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.default = with pkgs;
        mkShell {
          packages = with pkgs; [
            pnpm
            typescript-language-server
            vue-language-server
            cargo-tauri
            just
            watchexec
            systemfd
            cargo
            nodejs
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];

          shellHook = ''
            if ! [[ -d ./ui/node_modules ]]; then
              $(cd ui && pnpm install)
            fi
            PATH="$(pwd)/ui/node_modules/.bin:$PATH"
          '';
        };
    });
}
