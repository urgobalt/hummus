{
  description = "Build Hummus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [rust-overlay.overlays.default];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      projectName = "my-project"; # Choose a suitable name
      projectVersion = "0.1.0"; # Choose a version
      appBinaryName = pkgs.lib.strings.sanitizeDerivationName (import ./Cargo.toml).package.name;

      # === Dependency Fetching ===

      # Fetch pnpm dependencies based on ui/pnpm-lock.yaml
      # Run once with `lib.fakeSha256` or similar, then replace the hash.
      pnpmDeps = pkgs.fetchPnpmDeps {
        src = ./ui;
        hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
      };

      # Vendor Cargo dependencies for root, app, and server
      # Ensure all necessary Cargo.toml and Cargo.lock files are included in the src.
      # Run once with `lib.fakeSha256` or similar, then replace the hash.
      cargoDeps = pkgs.rustPlatform.fetchCargoTarball {
        src = ./.;
        # This hash needs to cover *all* crates needed by root, app/*, server/*
        sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
      };

      # === Main Derivation ===
      projectBuild = pkgs.stdenv.mkDerivation rec {
        pname = projectName;
        version = projectVersion;

        # Source filtering: Include only necessary files
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type: let
            baseName = baseNameOf path;
          in
            !(
              # Standard excludes
              baseName
              == ".git"
              || baseName == "result"
              || # Nix build result symlink
              baseName == ".direnv"
              || pkgs.lib.hasSuffix ".nix" baseName
              || # Exclude nix files themselves

              # Project specific build artifacts / caches
              baseName == "node_modules"
              || pkgs.lib.hasPrefix "${toString ./ui}/dist" (toString path)
              || pkgs.lib.hasPrefix "${toString ./app}/target" (toString path)
              || pkgs.lib.hasPrefix "${toString ./server}/target" (toString path)
              || baseName == ".env"
            );
        };

        # Define outputs corresponding to logical parts
        outputs = ["out" "ui" "app" "server"];

        # Tools needed during the build process itself
        nativeBuildInputs = with pkgs; [
          just
          nodejs_20 # Or pkgs.nodejs if system default is acceptable
          nodePackages.pnpm
          (rust-bin.nightly.latest.default.override {
            # Use rust-overlay for toolchain
            extensions = ["rust-src"]; # Needed for some builds, good practice
            # If tauri requires specific targets:
            # targets = ["...", "..."];
          })
          bash
          parallel
          pkg-config
          openssl # Often needed by Rust crates
          webkitgtk # For Tauri on Linux (adjust if using different GTK versions)
          gtk3 # Or gtk4 depending on webkitgtk version / Tauri config
          dbus
          glib # Runtime dependencies for webkitgtk
          # Required by webkitgtk (add others if build fails)
          libsoup
          cairo
          pango
          gdk-pixbuf
          atk
        ];

        # Point Cargo to the vendored dependencies and configure it
        cargoVendorDir = cargoDeps;
        postUnpack = ''
          # Create required .cargo/config.toml in source tree for cargo commands
          mkdir -p .cargo
          cat << EOF > .cargo/config.toml
          [source.crates-io]
          replace-with = "vendored-sources"

          [source.vendored-sources]
          directory = "${cargoDeps}/vendor"

          # Potentially add build script overrides if needed
          # [target.\'\'.*.build-override]
          # rustc = "${pkgs.rustPlatform.rust.rustc}/bin/rustc"
          EOF

          # Ensure cargo commands run within app/ and server/ also find the config
          # by copying it there. Alternatively, invoke cargo always from the root.
          # This copy assumes 'just' recipes might 'cd' into subdirs before cargo.
          cp .cargo/config.toml app/.cargo/config.toml
          cp .cargo/config.toml server/.cargo/config.toml

          # Set CARGO_HOME to prevent attempts to download into user's home
           export CARGO_HOME=$(mktemp -d)
        '';

        # Prepare pnpm dependencies before the main build
        preBuild = ''
          # pnpm needs a writable HOME, especially for cache/store if not configured otherwise
          export HOME=$(mktemp -d)

          # Enter the ui directory to prepare node_modules
          pushd ui
          echo "Preparing node_modules in $(pwd) using prefetched dependencies..."
          # Create node_modules from the prefetched store content
          pnpm install --offline --frozen-lockfile --prefer-offline --store-dir ${pnpmDeps}
          # Alternative: Copy/link from pnpmDeps - might be faster if large
          # mkdir -p node_modules
          # cp -r ${pnpmDeps}/node_modules/. ./node_modules/
          popd
        '';

        # Environment variables from justfile that influence builds
        # `export` isn't needed as mkDerivation sets them
        PARALLEL_SHELL = "${pkgs.bash}/bin/bash"; # Use nix provided bash
        NODE_PACKAGE_MANAGER = "pnpm"; # Keep consistent with justfile

        # Run the Just build commands
        buildPhase = ''
          runHook preBuild

          # Ensure Cargo uses the vendored sources explicitly if needed
          # (The .cargo/config.toml should handle this, but being explicit can help debug)
          # export CARGO_NET_OFFLINE=true

          echo "Building UI..."
          just build-ui
          echo "Building App..."
          just build-app
          echo "Building Server..."
          just build-server

          runHook postBuild
        '';

        # Copy build artifacts to the respective outputs
        installPhase = ''
          runHook preInstall

          # --- Install UI Output ---
          # Adjust destination path as needed for serving/consumption
          mkdir -p $ui/share/www
          cp -r ui/dist/* $ui/share/www/
          # Example: Add a simple index.html if dist is just assets
          # echo "<!DOCTYPE html><html><head><title>${projectName} UI</title></head><body>UI Assets</body></html>" > $ui/share/www/index.html

          # --- Install App Output ---
          mkdir -p $app/bin
          # Copy the main binary (adjust name if needed)
          cp app/target/release/${appBinaryName} $app/bin/
          # Copy other potential Tauri assets if they exist (e.g., icons, resources)
          # find app/target/release -maxdepth 1 -type f ! -name ${appBinaryName} -exec cp {} $app/bin/ \;
          # If there's an AppDir or similar structure created by cargo build --release:
          # cp -r app/target/release/bundle/AppDirOrSimilar $app/ # Example

          # --- Install Server Output ---
          mkdir -p $server/bin
          cp server/target/release/server $server/bin/server # Assuming binary is 'server'

          # --- Install Default Output ('out') ---
          # Let 'out' contain the server binary as a default convenience
          mkdir -p $out/bin
          ln -s $server/bin/server $out/bin/server

          runHook postInstall
        '';

        doCheck = false;

        meta = with pkgs.lib; {
          description = "Build outputs for ${projectName}";
          homepage = "https://example.com"; # Replace
          license = licenses.mit; # Replace with your actual license
          platforms = platforms.linux ++ platforms.darwin; # Adjust if needed
          mainProgram = "server"; # Default executable in 'out'
        };
      };
    in {
      # === Flake Outputs ===

      # The main package build derivation
      packages.default = projectBuild;

      # Individual output groups for convenience
      packages.ui = projectBuild.ui;
      packages.app = projectBuild.app;
      packages.server = projectBuild.server;

      # Development Shell (includes tools from justfile like watch exec)
      devShells.default = pkgs.mkShell {
        # Inherit build inputs, which include just, node, rust, etc.
        inputsFrom = [projectBuild];

        # Add tools needed for *development* (watch, etc.) if not already in build inputs
        packages = with pkgs; [
          systemfd
          watchexec
          # Add linters, formatters, etc. here
          # nodePackages.eslint # Example
          # rustfmt # Example
        ];

        # Environment variables for the dev shell
        shellHook = ''
          echo "Welcome to the ${projectName} dev environment!"
          # Environment variables from justfile
          export PARALLEL_SHELL="${pkgs.bash}/bin/bash"
          export NODE_PACKAGE_MANAGER="pnpm"
          # Set default port if needed by inner-watch-server directly
          # export DEFAULT_PORT=8080

          # Hint for user
          echo "Run 'just' to see available commands."

          # Optional: Configure pnpm store for the dev shell to avoid mixing with system
          export PNPM_STORE_PATH=$(pwd)/.pnpm-store

          # Set CARGO_HOME and .cargo/config for dev shell consistency
          export CARGO_HOME=$(pwd)/.cargo-home
          mkdir -p .cargo
          cp ${projectBuild.src}/.cargo/config.toml .cargo/config.toml || echo "No vendored config found, Cargo may fetch online."
        '';
      };
    });
}
