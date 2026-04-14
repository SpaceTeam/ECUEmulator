# ECU Emulator 
The goal of this is to create a virtual ECU, which emulates basic features of our ECU firmware.
This should help us in developing the rest of the software stack (LLServer, ECUI).

Note: This is still very much experimental and a work in progress.

## Setup

### Native (no Docker)

Add a virtual SocketCAN interface:
```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

### Docker / Docker Compose

The container **does not** create or manage any CAN interfaces. You must provide a SocketCAN interface from the outside (e.g. create `vcan0` on the host).

If you want the container to use a host `vcan0`, run it in the host network namespace.

#### Docker build + run (host networking)
```bash
docker build -t ecuemulator .

# create vcan0 on the host first
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan 2>/dev/null || true
sudo ip link set up vcan0

docker run --rm -it \
  --network host \
  -v "$(pwd)/config.toml:/config/config.toml:ro" \
  ecuemulator
```

Config handling:
- The container reads config from `${CONFIG_PATH}` (default: `/config/config.toml`).
- If you **don’t** bind-mount a config file, the container will seed `/config/config.toml` from `/config/sample_config.toml`.
- You can override the CAN interface at runtime with `CAN_INTERFACE`. Example: `-e CAN_INTERFACE=vcan42`.

#### Docker Compose
```bash
# seed a config in the repo root (first run)
cp -n data/sample_config.toml ./config.toml || true

# create vcan0 on the host first
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan 2>/dev/null || true
sudo ip link set up vcan0

docker compose up --build
```

## Development

### Running CI Checks

The repository includes a CI script (`ci-rust.sh`) that runs all quality checks on the Rust implementation. This script is used both locally and in GitHub Actions

**Run all checks:**
```bash
./ci-rust.sh
# or explicitly
./ci-rust.sh all
```

**Run individual checks:**
```bash
./ci-rust.sh build         # Build the project
./ci-rust.sh test          # Run tests
./ci-rust.sh fmt           # Check code formatting
./ci-rust.sh clippy        # Run clippy linter
```
You can fix formatting or linter issues by adding the -fix suffix to the command. e.g: `./ci-rust.sh clippy-fix`