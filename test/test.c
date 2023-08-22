#include "ckb_dlfcn.h"

static uint8_t g_code_buff[1024 * 1024] __attribute__((aligned(RISCV_PGSIZE)));

typedef int (*verify_t)(const uint8_t *, uint32_t, const uint8_t *, uint32_t,
                        const uint8_t *, uint32_t);

int main() {
  uint8_t data_hash[32];
  size_t len = 32;
  int ret = ckb_load_witness(data_hash, &len, 0, 0, CKB_SOURCE_INPUT);
  if (ret != 0) {
    return ret;
  }
  if (len != 32) {
    ckb_debug("Invalid data hash witness!");
    return 119;
  }

  uint8_t message[32];
  len = 32;
  ret = ckb_load_witness(message, &len, 0, 1, CKB_SOURCE_INPUT);
  if (ret != 0) {
    return ret;
  }
  if (len != 32) {
    ckb_debug("Invalid message witness!");
    return 119;
  }

  uint8_t public_key[65];
  len = 65;
  ret = ckb_load_witness(public_key, &len, 0, 2, CKB_SOURCE_INPUT);
  if (ret != 0) {
    return ret;
  }
  if (len != 65) {
    ckb_debug("Invalid public key witness!");
    return 119;
  }

  uint8_t signature[64];
  len = 65;
  ret = ckb_load_witness(signature, &len, 0, 3, CKB_SOURCE_INPUT);
  if (ret != 0) {
    return ret;
  }
  if (len != 64) {
    ckb_debug("Invalid signature witness!");
    return 119;
  }

  void *handle = NULL;
  size_t consumed_size = 0;
  ret = ckb_dlopen2(data_hash, 0, g_code_buff, 1024 * 1024, &handle,
                    &consumed_size);
  if (ret != 0) {
    return ret;
  }

  verify_t f = (verify_t)ckb_dlsym(handle, "verify111");
  if (f == NULL) {
    ckb_debug("Verify function missing!");
    return 119;
  }

  return f(public_key, 65, message, 32, signature, 64);
}
