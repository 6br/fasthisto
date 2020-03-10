#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include "rust_test.h"

int main(){
    const uint8_t* a = hello_rust();
    printf("quality: %d\n", *a);
}

/* calculate bin given an alignment covering [beg,end) (zero-based, half-closed-half-open) */
int reg2bin(int beg, int end){
    --end;if (beg>>14 == end>>14) return ((1<<15)-1)/7 + (beg>>14);if (beg>>17 == end>>17) return ((1<<12)-1)/7 + (beg>>17);if (beg>>20 == end>>20) return ((1<<9)-1)/7  + (beg>>20);if (beg>>23 == end>>23) return ((1<<6)-1)/7  + (beg>>23);if (beg>>26 == end>>26) return ((1<<3)-1)/7  + (beg>>26);return 0;
}
/* calculate the list of bins that may overlap with region [beg,end) (zero-based) */
#define MAX_BIN (((1<<18)-1)/7)
int reg2bins(int beg, int end, uint16_t list[MAX_BIN]){
    int i = 0, k;--end;list[i++] = 0;for (k =    1 + (beg>>26); k <=    1 + (end>>26); ++k) list[i++] = k;for (k =    9 + (beg>>23); k <=    9 + (end>>23); ++k) list[i++] = k;for (k =   73 + (beg>>20); k <=   73 + (end>>20); ++k) list[i++] = k;for (k =  585 + (beg>>17); k <=  585 + (end>>17); ++k) list[i++] = k;for (k = 4681 + (beg>>14); k <= 4681 + (end>>14); ++k) list[i++] = k;return i;
}