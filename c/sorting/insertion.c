#include <stdlib.h>

void sort_insertion(int *arr, size_t len) {
    if (len <= 1) { return; }
    int i = 1;
    while (i < len) {
        int j = i - 1;
        int element = arr[i];
        while (j >= 0 && arr[j] > element) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = element;
        i++;
    }
}