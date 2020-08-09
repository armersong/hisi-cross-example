#!/bin/sh

export RUST_TARGET_PATH=`pwd`
HOST_CC=gcc CC=arm-himix100-linux-gcc xargo build --target armv7-unknown-linux-uclibc --release
arm-himix100-linux-strip target/armv7-unknown-linux-uclibc/release/hello
