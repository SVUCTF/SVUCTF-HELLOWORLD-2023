#include <stdio.h>
#include <time.h>
#include <stdlib.h>

enum GAME
{
    EXIT,
    PLAY,
};

void init()
    {
        setvbuf(stdout,0,2,0);
        setvbuf(stdin,0,2,0);
        setvbuf(stderr,0,2,0);
    }

void menu() {

    puts("********************************");
    puts("+****** 猜数字游戏(1~100) *****+");
    puts("|*********** 0.EXIT ***********|");
    puts("+*********** 1.PLAY ***********+");
    puts("********************************");
}

void game() {

    int sj = rand() % 999 + 1;
    int cs = 0;
    int n = 0;

    while(++n < 3) {

        puts("Please Input:");
        scanf("%d", &cs);
        if (sj > cs)
            puts("It's too small!");
        else if (sj < cs)
            puts("It's too big!");
        else {
            puts("YOU ARE TRUE!");
            system("/bin/sh");
            break;
        }
    }
}

int main() {

    int input;

    init();
    srand(time(0));
    do {

        menu();
        scanf("%d",&input);
        switch(input) {
            case PLAY:
                game(); break;
            case EXIT:
                puts("Exit!"); break;
            default:
                puts("Inputs Error, Please Try Again!"); break;
        }

    } while(input);

    return 0;
}
