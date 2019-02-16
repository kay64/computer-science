#include <stdlib.h>
#include <string.h>

void sort_merge(int *arr, size_t len) {
    if (len <= 1) { return; }

    size_t split_len = len / 2;
    size_t left_len = split_len;
    size_t right_len = len - split_len;
    int *left = arr;
    int *right = (arr + split_len);
    sort_merge(left, left_len);
    sort_merge(right, right_len);

    int *buffer = calloc(sizeof(int), len);
    size_t i = 0, l = 0, r = 0;

    while (i < len) {
        if (l >= left_len) {
            memcpy(buffer + i, right + r, sizeof(int) * (right_len - r));
            break;
        }

        if (r >= right_len) {
            memcpy(buffer + i, left + l, sizeof(int) * (left_len - l));
            break;
        }

        if (left[l] < right[r]) {
            buffer[i] = left[l];
            l++;
        } else {
            buffer[i] = right[r];
            r++;
        }
        i++;
    }
    memcpy(arr, buffer, sizeof(int) * len);
    free(buffer);
}