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