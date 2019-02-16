#include <stdio.h>
#include "trees/red-black-tree.h"
#include "other/queue.h"
#include "sorting/sorting.h"

void print_red_black_node(RedBlackNode* node);

int main() {
    int arr[] = {64, 88, 49, 78, 99, 58, 52, 51, 36, 23, 16, 67, 14, 21, 55, 46, 19,
                 73, 12, 33, 44, 66, 32, 85, 71, 45, 24, 17, 40, 27, 18, 39, 59, 38,
                 97, 63, 83, 87, 6, 20, 56, 50, 43, 91, 34, 76, 31, 22, 28, 1};
    size_t len = 50;

    sort_btree(arr, len);

    int breakpoint;

//    RedBlackTree *tree = malloc(sizeof(RedBlackTree));
//    insert(tree, 11);
//    insert(tree, 2);
//    insert(tree, 14);
//    insert(tree, 15);
//    insert(tree, 1);
//    insert(tree, 7);
//    insert(tree, 5);
//    insert(tree, 8);
//    insert(tree, 4);
//
//    traverse_in_order(tree, &print_red_black_node);
//    printf("\n");
//    traverse_out_order(tree, &print_red_black_node);
//    printf("\n");
//    traverse_pre_order(tree, &print_red_black_node);
//    printf("\n");
//    traverse_post_order(tree, &print_red_black_node);
//    printf("\n");

    Queue *queue = new_queue();
    enqueue(queue, 1);
    enqueue(queue, 2);

    Option *option = dequeue(queue);
    printf("%d", option->value);
    dispose_queue(queue);
    return 0;
}

void print_red_black_node(RedBlackNode* node) {
    printf("%d ", node->value);
}

