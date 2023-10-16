#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void init() {
  setvbuf(stdout, 0, 2, 0);
  setvbuf(stdin, 0, 2, 0);
  setvbuf(stderr, 0, 2, 0);
}

void menu() {
  puts("********************************\n"
       "+****** 猜数字游戏(1~100) *****+\n"
       "********************************");
}

void game() {
  int sj = rand() % 999 + 1;
  int cs;

  puts("Please Input:");
  scanf("%d", &cs);
  if (sj > cs)
    puts("It's too small!");
  else if (sj < cs)
    puts("It's too big!");
  else {
    system("/bin/sh");
  }
}

int main() {
  srand(time(0));
  init();
  menu();
  game();
  return 0;
}
