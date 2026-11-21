#include <stdint.h>

__attribute__((noinline))
uint64_t sample_add(uint64_t a) {
    return a + 5;
}

__attribute__((noinline))
uint64_t sample_sub(uint64_t a) {
    return a - 4;
}

