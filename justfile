set shell := ["bash", "-uc"]
set dotenv-path := ".env"
export PARALLEL_SHELL := "bash"
export NODE_PACKAGE_MANAGER := "pnpm"

# List the commands
default:
  just --list

# Prepare node_modules
[private]
[group("ui")]
[working-directory: 'ui']
prepare-ui:
  if ! [[ -d node_modules ]]; then; $NODE_PACKAGE_MANAGER install; fi

# Build the ui
[private]
[group("ui")]
[working-directory: 'ui']
build-ui: prepare-ui
  $NODE_PACKAGE_MANAGER rsbuild build

# Build the ui in watch mode
[private]
[group("ui")]
[working-directory: 'ui']
watch-ui: prepare-ui
  $NODE_PACKAGE_MANAGER rsbuild build -w

# Launch ui in development mode
[private]
[group("ui")]
[working-directory: 'ui']
dev-ui: prepare-ui
  $NODE_PACKAGE_MANAGER rsbuild dev -w

# Build the native tauri application
[group("app")]
[working-directory: 'app']
build-app: build-ui
  cargo tauri build

# Start the native tauri application after building the ui and watch for changes
[group("app")]
[working-directory: 'app']
watch-app:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  just build-ui
  cargo tauri dev

# Build the server and ui
[group("web")]
[working-directory: 'server']
build-server: build-ui
  cargo build -q

# Start the webserver and watch for changes
[private]
[group("web")]
inner-watch-server:
  systemfd --no-pid -s http::$DEFAULT_PORT -- \
  watchexec --wrap-process none \
    -w api -w server \
    -e rs \
    -r -- cargo run -q --bin server

# Start the webserver after building the ui and watch for changes
[group("web")]
watch-server:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  just watch-ui
  just inner-watch-server

# Clean caches, outputs and more from the projects
[group("global")]
[confirm("This will remove all caches, dependencies and built binaries. Do you want to continue?")]
clean:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  rm ui/dist -rvd 2>/dev/null
  rm ui/node_modules/@rsbuild* -rvd 2>/dev/null
  rm ui/node_modules/@rspack* -rvd 2>/dev/null
  cargo clean 2>/dev/null

# Deploys development of all parts of the application and watches for changes
[group("global")]
watch-full-stack:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  just dev-ui
  cd app && cargo tauri dev
  just inner-watch-server
