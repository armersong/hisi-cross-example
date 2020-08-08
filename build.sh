#!/bin/sh

export RUST_TARGET_PATH=`pwd`
CC=arm-himix100-linux-gcc xargo build --target armv7-himix100-linux-gnueabihf --release
arm-himix100-linux-strip target/armv7-himix100-linux-gnueabihf/release/hello
