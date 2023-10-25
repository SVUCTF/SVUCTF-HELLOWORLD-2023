#include <stdio.h>
#include <time.h>
#include <stdlib.h>

int main() {
  int a;
  srand(time(0));
  a = rand() % 999 + 1;
  printf("%d",a);
  return 0;
}
