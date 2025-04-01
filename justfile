set shell := ["bash", "-uc"]
set dotenv-path := ".env"
export PARALLEL_SHELL := "bash"
export NODE_PACKAGE_MANAGER := "pnpm"
export GUM_CHOOSE_HEADER_FOREGROUND := "#B2854C"
export GUM_CHOOSE_CURSOR_FOREGROUND := "#858652"
export GUM_CHOOSE_ITEM_FOREGROUND := "#6D7577"

# List the commands
default:
  #!/usr/bin/env -S bash
  main=$(gum choose --header "Pick an action:" build dev docker clean)
  if ! [ $? = 0 ]; then
    exit 1
  fi
  if [ $main = build ]; then
    sub=$(gum choose --header "Pick a target:" app server ids)
    just build-$sub
  elif [ $main = dev ]; then
    sub=$(gum choose --header "Pick a target:" app server ids all)
    if [ $sub = all ]; then
      just watch-full-stack
    else
      just watch-$sub
    fi
  elif [ $main = docker ]; then
    command=$(gum choose --header "Pick a docker action:" run clean-run close build)
    service=$(gum choose --header "Pick a target" store ids)
    just docker-$command $service
  elif [ $main = clean ]; then
    just clean
  fi

# Prepare node_modules
[private]
[group("ui")]
[working-directory: 'ui']
prepare-ui:
  #!/usr/bin/env -S bash
  if ! [[ -d node_modules ]]; then
    $NODE_PACKAGE_MANAGER install
  fi

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
  cargo build --release

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
build-server: build-ui build-ids
  cargo build --release

# Start the webserver and watch for changes
[private]
[group("web")]
inner-watch-server:
  systemfd --no-pid -s http::$PORT -- \
  watchexec --wrap-process none \
    -w api -w server \
    -e rs \
    -r -- cargo run --bin server

# Start the webserver after building the ui and watch for changes
[group("web")]
watch-server:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  just watch-ui
  just inner-watch-server

# Build the ids server
[group("ids")]
[working-directory: 'ids']
build-ids:
  cargo build --release

# Build the service and start it within a docker container
[group("docker")]
docker-run service:
  sudo docker compose up -d --build -- {{service}}

# Build the service without chache and start it within a docker container
[group("docker")]
docker-clean-run service:
  sudo docker compose up -d --build --force-recreate -- {{service}}

# Close the services corresponding docker container
[group("docker")]
docker-close service:
  sudo docker compose down -- {{service}}

# Build the services docker container
[group("docker")]
docker-build service:
  sudo docker compose build --build -- {{service}}

# Clean caches, outputs and more from the projects
[group("global")]
[confirm("This will remove all caches, dependencies and built binaries. Do you want to continue?")]
clean:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  rm ui/dist -rvd 2>/dev/null
  cargo clean 2>/dev/null

# Deploys development of all parts of the application and watches for changes
[group("global")]
watch-full-stack:
  #!/usr/bin/env -S bash -- parallel --shebang --ungroup
  just dev-ui
  cd app && cargo tauri dev
  just inner-watch-server
