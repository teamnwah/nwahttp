#!/usr/bin/env bash
set -e
cd "$(dirname "$(realpath "$0")")"
docker build -t good-timer .

docker run -e RUST_BACKTRACE=full -p "25565:25565/udp" -p "8787:8787" -v "$PWD/config:/server/tes3mp-server-default.cfg" -v "$(realpath "$PWD/../target/debug/libnwahttp.so"):/server/data/scripts/nwahttp.so" -v "$PWD/data:/server/data" -ti good-timer
