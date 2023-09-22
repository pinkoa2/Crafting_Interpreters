#include <stdio.h>
#include <stdlib.h>

struct DoubleLinkedList {
  int length;
  struct Node *first;
  struct Node *last;
};

struct Node {
  int data;
  struct Node *prev;
  struct Node *next;
};

void insert(struct DoubleLinkedList *list, int pos, int data) {
  int length_of_list = list->length;

  struct Node *node = (struct Node *)malloc(sizeof(struct Node));
  node->data = data;

  if (length_of_list == 0) {
    list->first = node;
    list->last = node;
    list->length += 1;
    printf("None\n");
    return;
  }

  if (0 == pos) {
    struct Node *first_node = list->first;
    first_node->prev = node;
    node->next = first_node;
    list->first = node;
    list->length += 1;
    printf("First Pos\n");
    return;
  }

  if (length_of_list <= pos) {
    struct Node *last_node = list->last;
    last_node->next = node;
    node->prev = last_node;
    list->last = node;
    list->length += 1;
    printf("Last Pos\n");
    return;
  }

  printf("Inserting into middle\n");
  struct Node *node_of_interest = list->first;
  for (int i = 0; i < pos; i++) {
    node_of_interest = node_of_interest->next;
  }

  struct Node *parent = node_of_interest->prev;
  node_of_interest->prev = node;
  parent->next = node;
  node->prev = parent;
  node->next = node_of_interest;
  list->length += 1;
}

int find(struct DoubleLinkedList *list, int data) { 
  struct Node* node = list->first;
  for (int i = 0; i < list->length; i++) {
    if (node->data == data ) {
      return i;
    }
    node = node->next;
  }

  return -1; 
}


void clean(struct DoubleLinkedList* list) {
  struct Node* node = list->first;
  for (int i = 0; i < list->length; i++) {
    free(node->prev);
    node = node->next;
  }
  free(node);
  free(list);
}

int main() {

  struct DoubleLinkedList *list =
      (struct DoubleLinkedList *)malloc(sizeof(struct DoubleLinkedList));

  list->length = 0;
  insert(list, 0, 10);
  insert(list, 1, 3);
  insert(list, 2, 2);
  insert(list, 1, 1);
  insert(list, 10, 1);
  insert(list, 0, 5);

  struct Node *current = list->first;
  printf("Printing...\n");
  for (int i = 0; i < list->length; i++) {
    printf("%d\n", current->data);
    current = current->next;
  }

  printf("%d\n", find(list, 5));
  printf("%d\n", find(list, 1));
  printf("%d\n", find(list, 99));

  clean(list);
}

/*


[]
Insert 1
Insert 2 after 1
Insert



*/
