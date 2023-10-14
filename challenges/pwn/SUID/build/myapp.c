#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>


void init() {
    setvbuf(stdin, 0LL, 2, 0LL);
    setvbuf(stdout, 0LL, 2, 0LL);
    setvbuf(stderr, 0LL, 2, 0LL);
}

int main() { 
    int v1,v2;

    init();
    srand(10);
    v1 = rand() % 50;
    puts("Please Input Your Number:");
    scanf("%d", &v2);

    if (v2 == v1) {
        setuid(0);
        system("/bin/sh");
    } else {
        puts("HACKER!!!");
    }
    
    return 0;
} 
