#!/bin/bash
set -ex

CLANG="${CLANG:-clang-16}"

$CLANG --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
  -O3 -Wall -Werror \
  -ffunction-sections -fdata-sections -Wl,--gc-sections \
  -nostdinc -nostdlib -I ckb-c-stdlib -I ckb-c-stdlib/libc \
  -DCKB_C_STDLIB_PRINTF \
  test.c -o test

cd runner
cargo test -- --nocapture
