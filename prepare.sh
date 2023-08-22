#!/bin/bash
set -ex

cd p256
rustup target add riscv64imac-unknown-none-elf
rustup component add rust-src
