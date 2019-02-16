#include <stdbool.h>
#include <stdlib.h>
#include "red-black-tree.h"

static RedBlackNode *_create_node(int value) {
    RedBlackNode *node = malloc(sizeof(RedBlackNode));
    node->color = RED;
    node->value = value;
    node->left = NULL;
    node->right = NULL;
    return node;
}

static void _recolor(RedBlackNode *node, NodeColor color) {
    node->color = color;
    NodeColor child_color = color == RED ? BLACK : RED;
    if (node->left != NULL) {
        _recolor(node->left, child_color);
    }
    if (node->right != NULL) {
        _recolor(node->right, child_color);
    }
}

static void _rotate_left(RedBlackNode *node, RedBlackNode *parent, RedBlackNode *grandparent) {
    if (grandparent != NULL) {
        if (grandparent->right == parent) {
            grandparent->right = node;
        } else {
            grandparent->left = node;
        }
    }
    node->parent = grandparent;

    parent->right = node->left;
    if (node->left != NULL) {
        node->left->parent = parent;
    }

    node->left = parent;
    parent->parent = node;
}

static void _rotate_right(RedBlackNode *node, RedBlackNode *parent, RedBlackNode *grandparent) {
    if (grandparent != NULL) {
        if (grandparent->right == parent) {
            grandparent->right = node;
        } else {
            grandparent->left = node;
        }
    }

    node->parent = grandparent;

    parent->left = node->right;
    if (node->right != NULL) {
        node->right->parent = parent;
    }

    node->right = parent;
    parent->parent = node;
}

static void _swap(RedBlackNode **n1, RedBlackNode **n2) {
    RedBlackNode *tmp = *n1;
    *n1 = *n2;
    *n2 = tmp;
}

static void _balance(RedBlackTree *tree, RedBlackNode *node) {
    RedBlackNode *parent = node->parent;
    while (parent != NULL && parent->color == RED) {
        RedBlackNode *grandparent = parent->parent;
        if (grandparent == NULL) {
            break;
        }
        RedBlackNode *uncle = grandparent->left == parent
                              ? grandparent->right
                              : grandparent->left;
        if (uncle != NULL && uncle->color == RED) {
            //uncle is red - need to recolor
            grandparent->color = RED;
            parent->color = uncle->color = BLACK;
            node = grandparent;
            parent = node->parent;
        } else {
            //uncle is black - need to rotate
            if (grandparent->left == parent) {
                if (parent->right == node) {
                    _rotate_left(node, parent, grandparent);
                    _swap(&node, &parent);
                }
                parent->color = BLACK;
                grandparent->color = RED;
                _rotate_right(parent, grandparent, grandparent->parent);
            } else {
                if (parent->left == node) {
                    _rotate_right(node, parent, grandparent);
                    _swap(&node, &parent);
                }
                parent->color = BLACK;
                grandparent->color = RED;
                _rotate_left(parent, grandparent, grandparent->parent);
            }
        }
    }
    if (parent == NULL) {
        tree->root = node;
        tree->root->color = BLACK;
    } else {
        if (parent->parent == NULL) {
            tree->root = parent;
            tree->root->color = BLACK;
        }
    }
}

static void _insert_leaf(RedBlackTree *tree, RedBlackNode *node) {
    RedBlackNode *parent = tree->root;
    bool should_continue = true;
    int value = node->value;
    do {
        if (value > parent->value) {
            if (parent->right == NULL) {
                parent->right = node;
                node->parent = parent;

                should_continue = false;
            } else {
                parent = parent->right;
            }
        } else {
            if (parent->left == NULL) {
                parent->left = node;
                node->parent = parent;

                should_continue = false;
            } else {
                parent = parent->left;
            }
        }
    } while (should_continue);

    _balance(tree, node);
}

void rbt_insert(RedBlackTree *tree, int value) {
    RedBlackNode *node = _create_node(value);
    if (tree->root == NULL) {
        tree->root = node;
        node->color = BLACK;
        return;
    }

    _insert_leaf(tree, node);
}

RedBlackNode *rbt_find(RedBlackTree *tree, int value) {
    RedBlackNode *cur = tree->root;
    while (cur != NULL && cur->value != value) {
        if (cur->value > value) {
            cur = cur->left;
        } else {
            cur = cur->right;
        }
    }

    return cur;
}

bool rbt_contains(RedBlackTree *tree, int value) {
    return rbt_find(tree, value) != NULL;
}

static void _traverse_pre_order(RedBlackNode *node, void callback(RedBlackNode *)) {
    if (node == NULL) {
        return;
    }
    callback(node);
    _traverse_pre_order(node->left, callback);
    _traverse_pre_order(node->right, callback);
}

void rbt_traverse_pre_order(RedBlackTree *tree, void callback(RedBlackNode *)) {
    _traverse_pre_order(tree->root, callback);
}

static void _traverse_in_order(RedBlackNode *node, void callback(RedBlackNode *)) {
    if (node == NULL) {
        return;
    }
    _traverse_in_order(node->left, callback);
    callback(node);
    _traverse_in_order(node->right, callback);
}

void rbt_traverse_in_order(RedBlackTree *tree, void callback(RedBlackNode *)) {
    _traverse_in_order(tree->root, callback);
}


static void _traverse_post_order(RedBlackNode *node, void callback(RedBlackNode *)) {
    if (node == NULL) {
        return;
    }
    _traverse_post_order(node->left, callback);
    _traverse_post_order(node->right, callback);
    callback(node);
}

void rbt_traverse_post_order(RedBlackTree *tree, void callback(RedBlackNode *)) {
    _traverse_post_order(tree->root, callback);
}

static void _traverse_out_order(RedBlackNode *node, void callback(RedBlackNode *)) {
    if (node == NULL) {
        return;
    }
    _traverse_out_order(node->right, callback);
    callback(node);
    _traverse_out_order(node->left, callback);
}

void rbt_traverse_out_order(RedBlackTree *tree, void callback(RedBlackNode *)) {
    _traverse_out_order(tree->root, callback);
}

void traverse_bfs(RedBlackTree *tree, void callback(RedBlackNode *)) {

}

static void _dispose(RedBlackNode *node) {
    free(node);
}

void rbt_dispose(RedBlackTree *tree) {
    rbt_traverse_post_order(tree, &_dispose);
    free(tree);
}

RedBlackTree *rbt_new() {
    return malloc(sizeof(RedBlackTree));
}
