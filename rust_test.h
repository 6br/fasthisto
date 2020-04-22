#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>
#include <string.h>

static const uint16_t MAX_BIN = 37448;

struct Region;

struct String;

template<typename T>
struct Vec;

extern "C" {

void bam(const char *c_buf);

const uint8_t *bit_packing(const uint32_t *ptr, uintptr_t len);

const uint8_t *compress_bytes_extern(uint8_t words[8]);

const uint8_t *compress_bytes_extern_unwrap(uint8_t words[8]);

Region decrement_start(Region region);

const uint8_t *deflate(uint8_t data[16]);

const uint8_t *float_compression(double data[4]);

const uint8_t *hello_rust();

Vec<Feature> load_bigbed(String path, Region region);

void start_minus(Region *self);

} // extern "C"
