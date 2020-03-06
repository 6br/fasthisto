#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Region;

template<typename T, typename E>
struct Result;

extern "C" {

Region decrement_start(Region region);

Result fmt(const Region *self, Formatter *f);

const uint8_t *hello_rust();

void start_minus(Region *self);

} // extern "C"
