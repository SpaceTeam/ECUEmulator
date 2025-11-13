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

