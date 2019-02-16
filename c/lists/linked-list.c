typedef struct Element_ {
    int value;
    struct Element_ next;
} Element;

typedef struct LinkedList_ {
    Element *head
} LinkedList;

