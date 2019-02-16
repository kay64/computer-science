#include <stdbool.h>

typedef struct QueueElement_ {
    int value;
    struct QueueElement_ *next;
} QueueElement;

typedef struct Queue_ {
    QueueElement *head;
} Queue;

typedef struct Option_ {
    int value;
    bool is_null;
} Option;

Queue *new_queue();

void dispose_queue(Queue *queue);

void enqueue(Queue *queue, int value);

Option *dequeue(Queue *queue);

Option *pick(Queue *queue);