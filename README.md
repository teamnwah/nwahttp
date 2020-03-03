# nwahttp

Putting things in places where they shouldn't be

## Features

- Prometheus metrics endpoint
- REST API with player info
- WebSocket with realtime player info

## Usage

build with `cargo build --release` for target and place `nwahttp.so` in the `$TES3MP_HOME/scripts` folder, and add `nwahttp.so` to the scripts argument in the config,

then connect via `http://[ip of tes3mp server]:8787`
