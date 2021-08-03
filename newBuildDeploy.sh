#!/bin/bash
docker compose up && scp ./target/arm-unknown-linux-gnueabihf/release/camera_rig pi@192.168.1.10:/home/pi/camera-rig
rm -rf ./target