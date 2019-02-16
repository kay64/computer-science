#include <stdlib.h>
#include "helpers.h"

void array_swap(int *arr, size_t index1, size_t index2) {
    int tmp = *(arr + index1);
    *(arr + index1) = *(arr + index2);
    *(arr + index2) = tmp;
}