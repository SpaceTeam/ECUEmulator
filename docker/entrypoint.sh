#!/bin/sh
set -eu

# Configurable CAN interface name (defaults to vcan0)
VCAN_IFACE="${VCAN_IFACE:-vcan0}"
CONFIG_PATH="${CONFIG_PATH:-/config/config.toml}"
PATCH_CONFIG_CAN_IFACE="${PATCH_CONFIG_CAN_IFACE:-1}"

# Create vcan interface inside the container network namespace.
# Requires: docker run --cap-add=NET_ADMIN ...
if ! ip link show "$VCAN_IFACE" >/dev/null 2>&1; then
  ip link add dev "$VCAN_IFACE" type vcan
fi
ip link set up "$VCAN_IFACE"

# If no config exists (e.g. first run without a bind mount), seed it from sample.
if [ ! -f "$CONFIG_PATH" ] && [ -f /config/sample_config.toml ]; then
  mkdir -p "$(dirname "$CONFIG_PATH")"
  cp /config/sample_config.toml "$CONFIG_PATH"
fi

# Keep config and created interface in sync by patching can_interface in the TOML.
# If the config is bind-mounted read-only, we can't edit it in place. In that case,
# copy it to a writable temp file and use that instead.
if [ "$PATCH_CONFIG_CAN_IFACE" = "1" ] && [ -f "$CONFIG_PATH" ]; then
  if [ ! -w "$CONFIG_PATH" ]; then
    tmp_cfg="/tmp/config.toml"
    cp "$CONFIG_PATH" "$tmp_cfg"
    CONFIG_PATH="$tmp_cfg"
  fi
  if grep -qE '^[[:space:]]*can_interface[[:space:]]*=' "$CONFIG_PATH"; then
    # Replace existing can_interface = "..."
    sed -i -E "s@^([[:space:]]*can_interface[[:space:]]*=[[:space:]]*)\".*\"@\1\"$VCAN_IFACE\"@" "$CONFIG_PATH"
  else
    # Add it near the top if not present
    tmp="$(mktemp)"
    {
      echo "can_interface = \"$VCAN_IFACE\""
      cat "$CONFIG_PATH"
    } > "$tmp"
    cat "$tmp" > "$CONFIG_PATH"
    rm -f "$tmp"
  fi
fi

# If the user did not provide an explicit command, run the emulator with CONFIG_PATH.
if [ "$#" -eq 0 ]; then
  exec /usr/local/bin/ecuemulator "$CONFIG_PATH"
fi

exec "$@"
