#include <stdlib.h>

static size_t _get_minrun(size_t len) {
    int r = 0;
    while (len >= 64) {
        r |= len & 1;
        len >>= 1;
    }

    return r + len;
}

void sort_timsort(int *arr, size_t len) {
    size_t i = 0;
    
}