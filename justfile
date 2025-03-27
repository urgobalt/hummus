set shell := ["bash", "-c"]

# List the commands
default:
  just --list

# Build the ui
[working-directory: 'ui']
build-ui:
  rsbuild build -w

# Build the server and ui
[working-directory: 'server']
build-server: build-ui
  cargo build -q

[working-directory: 'app']
build-app:
  cargo tauri build

# Start the webserver after building the ui
[working-directory: 'server']
serve:
  cargo run -q

# Start the webserver after building the ui and watch for changes
watch-serve:
  just build-ui &
  systemfd --no-pid -s http::3000 -- \
  watchexec \
    --wrap-process none \
    -w api \
    -w server \
    -e rs -r \
    just serve

# Start the app after building the ui and watch for changes
[working-directory: 'hulp']
watch-tauri:
  watchexec \
    --wrap-process none \
    -w ui \
    -e html,css,js -r \
    just build-ui &
  cargo tauri dev

# Clean caches, outputs and more from the projects
clean:
  rm ui/dist -rd &2>/dev/null
  rm ui/node_modules/.vite* -rd &2>/dev/null
  rm server/target* -rd &2>/dev/null
  rm app/target* -rd &2>/dev/null
  cd server && cargo clean &2>/dev/null
  cd app && cargo clean &2>/dev/null
  cargo clean &2>/dev/null
