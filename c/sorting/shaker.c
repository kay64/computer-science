#include <lzma.h>
#include "helpers.h"

void sort_shaker(int *arr, size_t len) {
    if(len <= 1) { return; }
    size_t start = 0;
    size_t end = len - 1;
    size_t index = start;
    while (start < end) {
        while (index < end) {
            if (*(arr + index) > *(arr + index + 1)) {
                array_swap(arr, index, index + 1);
            }
            index++;
        }
        end--;
        index--;
        while (index > start) {
            if (*(arr + index) < *(arr + index - 1)) {
                array_swap(arr, index, index - 1);
            }
            index--;
        }
        start++;
        index++;
    }
}