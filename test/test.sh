#!/bin/bash
set -ex

# Inspired from https://stackoverflow.com/a/246128
TOP="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $TOP

CLANG="${CLANG:-clang-16}"

$CLANG --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
  -O3 -Wall -Werror \
  -ffunction-sections -fdata-sections -Wl,--gc-sections \
  -nostdinc -nostdlib -I ckb-c-stdlib -I ckb-c-stdlib/libc \
  -DCKB_C_STDLIB_PRINTF \
  test.c -o test

cd runner
cargo test -- --nocapture
