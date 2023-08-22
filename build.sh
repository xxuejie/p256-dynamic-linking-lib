#!/bin/bash
set -ex

AR="${AR:-llvm-ar-16}"
OBJCOPY="${OBJCOPY:-llvm-objcopy-16}"
LD="${LD:-ld.lld-16}"
STRIP="${STRIP:-llvm-strip-16}"

cd p256
cargo build --release --target=riscv64imac-unknown-none-elf -Z build-std=core
cd ..

# For some reason, lld cannot be used to link against the archive file generated
# by Rust compilers directly, we will have to extract the archive file into
# individual objects, and link against those objects directly. It might be due
# to the fact that some CLI arguments are not tweaked correctly. For now we will
# simply extract the archive file.
rm -rf objects
mkdir -p objects

cd objects
$AR x ../p256/target/riscv64imac-unknown-none-elf/release/libp256_dynamic_lib.a
cd ..

$LD --shared --gc-sections --dynamic-list lib.syms -o p256.so objects/*.o
$STRIP p256.so -o p256-striped.so
