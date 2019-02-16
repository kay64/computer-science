#include <stdbool.h>
#include <stdlib.h>

typedef enum {
    RED, BLACK
} NodeColor;

typedef struct RedBlackNode_ {
    NodeColor color;
    int value;
    struct RedBlackNode_ *right;
    struct RedBlackNode_ *left;
    struct RedBlackNode_ *parent;
} RedBlackNode;

typedef struct {
    RedBlackNode *root;
} RedBlackTree;

RedBlackTree *rbt_new();

void rbt_dispose(RedBlackTree *tree);

void rbt_insert(RedBlackTree *tree, int value);

RedBlackNode *rbt_find(RedBlackTree *tree, int value);

bool rbt_contains(RedBlackTree *tree, int value);



void rbt_traverse_pre_order(RedBlackTree *tree, void callback(RedBlackNode *));

void rbt_traverse_in_order(RedBlackTree *tree, void callback(RedBlackNode *));

void rbt_traverse_post_order(RedBlackTree *tree, void callback(RedBlackNode *));

void rbt_traverse_out_order(RedBlackTree *tree, void callback(RedBlackNode *));

void rbt_traverse_bfs(RedBlackTree *tree, void callback(RedBlackNode *));