#include <stdlib.h>
#include "../trees/binary-tree.h"

static void _traverse(BinaryTreeNode *node, int* arr, size_t *index) {
    if (node != NULL) {
        _traverse(node->left, arr, index);
        arr[*index] = node->value;
        (*index)++;
        _traverse(node->right, arr, index);
    }
}

void sort_btree(int *arr, size_t len) {
    if (len <= 1) { return; }

    BinaryTree *tree = bt_new();
    for (size_t i = 0; i < len; i++) {
        bt_insert(tree, arr[i]);
    }

    size_t  index = 0;

    _traverse(tree->root, arr, &index);
    bt_dispose(tree);
}

