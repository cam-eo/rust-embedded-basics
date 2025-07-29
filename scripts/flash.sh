#!/usr/bin/env bash
set -euo pipefail

# ---------------------------
# Defaults
# ---------------------------
PORT="${ESP_PORT:-}"      # you can also export ESP_PORT to skip passing -p every time
MODE="debug"              # or "release"
MONITOR=1                 # open monitor by default
CLEAN=0                   # don't cargo clean by default

# ---------------------------
# Helpers
# ---------------------------
usage() {
  cat <<'EOF'
Usage: flash.sh [options]

Options:
  -p, --port <path>   Serial port to use (e.g. /dev/ttyACM0). If omitted, the script will try to auto-detect.
  -r, --release       Build in release mode (default is debug)
  -d, --debug         Build in debug mode (default)
  -n, --no-monitor    Do not open the serial monitor after flashing
  -c, --clean         Run `cargo clean` before building
  -h, --help          Show this help and exit

Environment:
  ESP_PORT            Same as --port
  ESP_EXPORT          Path to export-esp.sh (defaults to ~/export-esp.sh if present)

Examples:
  ./scripts/flash.sh -r
  ./scripts/flash.sh -p /dev/ttyACM0
  ESP_PORT=/dev/ttyUSB0 ./scripts/flash.sh -r -n
EOF
}

log() { echo -e "\033[1;34m[flash]\033[0m $*"; }

# ---------------------------
# Parse args
# ---------------------------
while [[ $# -gt 0 ]]; do
  case "$1" in
    -p|--port) PORT="$2"; shift 2;;
    -r|--release) MODE="release"; shift;;
    -d|--debug) MODE="debug"; shift;;
    -n|--no-monitor) MONITOR=0; shift;;
    -c|--clean) CLEAN=1; shift;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown option: $1" >&2; usage; exit 1;;
  esac
done

# ---------------------------
# Source esp toolchain env
# ---------------------------
ESP_EXPORT="${ESP_EXPORT:-$HOME/export-esp.sh}"
if [[ -f "$ESP_EXPORT" ]]; then
  log "Sourcing $ESP_EXPORT"
  # shellcheck disable=SC1090
  source "$ESP_EXPORT"
else
  log "WARNING: $ESP_EXPORT not found. Assuming your environment is already configured."
fi

# ---------------------------
# Optionally clean
# ---------------------------
if [[ $CLEAN -eq 1 ]]; then
  log "cargo clean"
  cargo clean
fi

# ---------------------------
# Build
# ---------------------------
if [[ "$MODE" == "release" ]]; then
  log "cargo build --release"
  cargo build --release
else
  log "cargo build"
  cargo build
fi

# ---------------------------
# Determine binary path
# ---------------------------
BIN_PATH="target/xtensa-esp32s3-espidf/$MODE/audio-controller"
if [[ ! -f "$BIN_PATH" ]]; then
  log "ERROR: Built binary not found at $BIN_PATH"
  exit 1
fi

# ---------------------------
# Detect port if not passed
# ---------------------------
if [[ -z "$PORT" ]]; then
  for candidate in /dev/ttyACM* /dev/ttyUSB*; do
    if [[ -e "$candidate" ]]; then
      PORT="$candidate"
      break
    fi
  done

  if [[ -z "$PORT" ]]; then
    log "ERROR: Could not auto-detect serial port. Pass it with --port or set ESP_PORT."
    exit 1
  fi
fi

log "Using port: $PORT"

# ---------------------------
# Flash and (maybe) monitor
# ---------------------------
if [[ $MONITOR -eq 1 ]]; then
  log "Flashing and opening monitor..."
  espflash flash --port "$PORT" "$BIN_PATH" --monitor
else
  log "Flashing (no monitor)..."
  espflash flash --port "$PORT" "$BIN_PATH"
fi
