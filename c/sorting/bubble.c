#include <glob.h>
#include "helpers.h"
#include "sorting.h"

void sort_bubble(int *arr, size_t len) {
    if (len <= 1) { return; }
    size_t end = len - 1;
    while (end > 0) {
        size_t index = 0;
        while (index < end) {
            if (*(arr + index) > *(arr + index + 1)) {
                array_swap(arr, index, index + 1);
            }
            index++;
        }
        end--;
    }
}