#include <stdlib.h>
#include "helpers.h"

void sort_gnome(int *arr, size_t len) {
    if (len <= 1) { return; }
    size_t i = 1;
    size_t anchor = i;
    while (i < len) {
        if (arr[i] > arr[i - 1]) {
            if (i < anchor) {
                anchor++;
                i = anchor;
            } else {
                i++;
                anchor = i;
            }
        } else {
            array_swap(arr, i, i - 1);
            i--;
        }
    }
}