#!/bin/bash
docker-compose up && scp ./target/arm-unknown-linux-gnueabihf/release/camera_rig pi@192.168.0.25:/home/pi/camera-rig
sudo rm -rf ./target