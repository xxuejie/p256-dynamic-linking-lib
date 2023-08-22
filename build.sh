#!/bin/bash
set -ex

AR="${AR:-llvm-ar-16}"
OBJCOPY="${OBJCOPY:-llvm-objcopy-16}"
LD="${LD:-ld.lld-16}"
STRIP="${STRIP:-llvm-strip-16}"

cd p256
cargo build --release --target=riscv64imac-unknown-none-elf -Z build-std=core
cd ..
./process_lib.sh ./p256/target/riscv64imac-unknown-none-elf/release/libp256_dynamic_lib.a lib.syms objects $AR $OBJCOPY
$LD --shared --gc-sections --dynamic-list lib.syms -o p256.so objects/*.o
$STRIP p256.so -o p256-striped.so
