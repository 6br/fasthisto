#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio>
#include <string>

typedef struct Region Region;

void bam(const char *c_buf);

const uint8_t *bit_packing(const uint32_t *ptr, uintptr_t len);

const uint8_t *compress_bytes_extern(uint8_t words[8]);

Region decrement_start(Region region);

const uint8_t *deflate(uint8_t data[16]);

const uint8_t *float_compression(double data[4]);

const uint8_t *hello_rust(void);

void start_minus(Region *self);
