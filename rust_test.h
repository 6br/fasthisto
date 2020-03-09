#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct String String;

typedef struct {
  String path;
  uint64_t start;
  uint64_t stop;
} Region;

const uint8_t *bit_packing(const uint32_t *ptr, uintptr_t len);

Region decrement_start(Region region);

const uint8_t *hello_rust(void);

void start_minus(Region *self);
