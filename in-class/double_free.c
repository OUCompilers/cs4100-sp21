#include <stdio.h>
#include <stdlib.h>

typedef struct Dummy { int a; int b; } Dummy;

void foo(void) {
  Dummy *ptr = malloc(sizeof(struct Dummy));
  Dummy *alias = ptr;
  free(ptr);
  int a = alias->a;
  free(alias);
}

int main() {
  for (int i = 0; i < 10; ++i) {
    foo();
  }
  printf("hello\n");
}
