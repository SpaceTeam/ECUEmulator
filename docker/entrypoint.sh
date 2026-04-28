#!/bin/sh
set -eu

CONFIG_PATH="${CONFIG_PATH:-/config/config.toml}"

# If no config exists (e.g. first run without a bind mount), seed it from sample.
# Note: if CONFIG_PATH points into a read-only mount, this will fail (as intended).
if [ ! -f "$CONFIG_PATH" ] && [ -f /config/sample_config.toml ]; then
  mkdir -p "$(dirname "$CONFIG_PATH")"
  cp /config/sample_config.toml "$CONFIG_PATH"
fi

# If the user did not provide an explicit command, run the emulator with CONFIG_PATH.
if [ "$#" -eq 0 ]; then
  exec /usr/local/bin/ecuemulator "$CONFIG_PATH"
fi

exec "$@"
