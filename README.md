# ECU Emulator 
The goal of this is to create a virtual ECU, which emulates basic features of our ECU firmware.
This should help us in developing the rest of the software stack (LLServer, ECUI).

Note: This is still very much experimental and a work in progress.

## Setup

add virtual socketcan interface:
```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
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
./ci-rus/t.sh clippy        # Run clippy linter
```
You can fix formatting or linter issues by adding the -fix suffix to the command. e.g: `./ci-rust.sh clippy-fix`