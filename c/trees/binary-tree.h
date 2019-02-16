#include <stdbool.h>

typedef struct BinaryTreeNode_ {
    int value;
    struct BinaryTreeNode_ *right;
    struct BinaryTreeNode_ *left;
    struct BinaryTreeNode_ *parent;
} BinaryTreeNode;

typedef struct {
    BinaryTreeNode *root;
} BinaryTree;

BinaryTree *bt_new();

void bt_dispose(BinaryTree *tree);

void bt_insert(BinaryTree *tree, int value);

BinaryTreeNode *bt_find(BinaryTree *tree, int value);

bool bt_contains(BinaryTree *tree, int value);
