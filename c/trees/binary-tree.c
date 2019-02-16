#include <stdbool.h>
#include <malloc.h>
#include "binary-tree.h"

static void _dispose(BinaryTreeNode *node) {
    free(node);
}

static void _traverse_post_order(BinaryTreeNode *node, void callback(BinaryTreeNode *)) {
    if (node == NULL) {
        return;
    }
    _traverse_post_order(node->left, callback);
    _traverse_post_order(node->right, callback);
    callback(node);
}

static BinaryTreeNode *_create_node(int value) {
    BinaryTreeNode *node = malloc(sizeof(BinaryTreeNode));
    node->value = value;
    return node;
}

BinaryTree *bt_new() {
    return malloc(sizeof(BinaryTree));
}

void bt_dispose(BinaryTree *tree) {
    if (tree->root != NULL) {
        _traverse_post_order(tree->root, &_dispose);
    }
    free(tree);
}

void bt_insert(BinaryTree *tree, int value) {
    BinaryTreeNode *node = _create_node(value);
    if (tree->root == NULL) {
        tree->root = node;
        return;
    }
    BinaryTreeNode *curr = tree->root;
    while (1) {
        if (value > curr->value) {
            if (curr->right == NULL) {
                curr->right = node;
                break;
            } else {
                curr = curr->right;
            }
        } else {
            if (curr->left == NULL) {
                curr->left = node;
                break;
            } else {
                curr = curr->left;
            }
        }
    }
}

BinaryTreeNode *bt_find(BinaryTree *tree, int value) {
    BinaryTreeNode *node = tree->root;
    while (node != NULL) {
        if (node->value == value) {
            return node;
        } else if (node->value > value) {
            node = node->left;
        } else {
            node = node->right;
        }
    }

    return node;
}

bool bt_contains(BinaryTree *tree, int value) {
    return bt_find(tree, value) != NULL;
}