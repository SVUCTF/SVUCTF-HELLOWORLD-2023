#include <stdio.h>
#include <stdlib.h>
#include <time.h>

enum GAME {
  EXIT,
  PLAY,
};

void init() {
  setvbuf(stdout, 0, 2, 0);
  setvbuf(stdin, 0, 2, 0);
  setvbuf(stderr, 0, 2, 0);
}

void menu() {
  puts("********************************\n"
       "+****** 猜数字游戏(1~100) *****+\n"
       "|*********** 0.EXIT ***********|\n"
       "+*********** 1.PLAY ***********+\n"
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

  int input;

  init();
  menu();
  srand(time(0));
  scanf("%d", &input);
  switch (input) {
  case PLAY:
    game();
    break;
  case EXIT:
    puts("Exit!");
    break;
  default:
    puts("Inputs Error, Please Try Again!");
    break;
  }

  return 0;
}
