#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Region Region;

Region decrement_start(Region region);

const uint8_t *hello_rust(void);

void start_minus(Region *self);
