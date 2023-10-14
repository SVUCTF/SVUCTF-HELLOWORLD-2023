#include <stdio.h>
#include <stdlib.h>

int main() { 
    int v1,v2;

    srand(10);
    v1 = rand() % 50;
    puts("Please Input Your Number:");
    scanf("%d", &v2);

    if (v2 == v1)
        system("/bin/sh");
    else
        puts("HACKER!!!");

    return 0;
} 
