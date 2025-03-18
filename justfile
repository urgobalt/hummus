set shell := ["bash", "-c"]

# List the commands
default:
  just --list

# Build huui; the main website component
[working-directory: 'huui']
build-huui:
  pnpm vite build

# Build huttp; the webserver and huui
[working-directory: 'huttp']
build-huttp: build-huui
  cargo build

[working-directory: 'hulp']
build-tauri:
  cargo tauri build

# Start huttp (the webserver) after building the frontend component (huui)
[working-directory: 'huttp']
serve-huttp: build-huui
  cargo run

# Start huttp (the webserver) after building the frontend component (huui) and watch for changes
watch-serve:
  systemfd --no-pid -s http::3000 -- \
  watchexec \
    --wrap-process none \
    -w huui \
    -w huttp \
    -e html,css,js,rs -r \
    just serve-huttp

# Start huttp (the webserver) after building the frontend component (huui) and watch for changes
[working-directory: 'hulp']
watch-tauri:
  cargo tauri dev &
  just watch-serve

# Clean caches, outputs and more from the projects
clean:
  rm huui/dist -rd &2>/dev/null
  rm huui/node_modules/.vite* -rd &2>/dev/null
  cd huttp && cargo clean &2>/dev/null
  cd hulp && cargo clean &2>/dev/null
