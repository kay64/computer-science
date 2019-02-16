#include <stdlib.h>
#include "queue.h"

Queue *new_queue() {
    return malloc(sizeof(Queue));
}

void dispose_queue(Queue *queue) {
    QueueElement *cur = queue->head;
    while (cur != NULL) {
        QueueElement *next = cur->next;
        free(cur);
        cur = next;
    }
    free(queue);
}

QueueElement *_create_element(int value) {
    QueueElement *element = malloc(sizeof(QueueElement));
    element->value = value;
    return element;
}

void enqueue(Queue *queue, int value) {
    if (queue->head == NULL) {
        queue->head = _create_element(value);
    } else {
        QueueElement *tail = queue->head;
        while (tail->next != NULL) {
            tail = tail->next;
        }
        tail->next = _create_element(value);
    }
}

Option *dequeue(Queue *queue) {
    QueueElement *element = queue->head;

    Option *result = malloc(sizeof(Option));
    if (element == NULL) {
        result->is_null = true;
    } else {
        result->value = element->value;
        queue->head = element->next;
        free(element);
    }

    return result;
}