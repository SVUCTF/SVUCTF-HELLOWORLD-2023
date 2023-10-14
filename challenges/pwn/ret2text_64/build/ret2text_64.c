#include <stdio.h>
#include <stdlib.h>


char secret[] = "/bin/sh";

void init() {
    setvbuf(stdin, 0LL, 2, 0LL);
    setvbuf(stdout, 0LL, 2, 0LL);
    setvbuf(stderr, 0LL, 2, 0LL);
}

void banner() {
    puts("---------------------------------------------------\n"
         "███████╗██╗   ██╗██╗   ██╗ ██████╗████████╗███████╗\n"
         "██╔════╝██║   ██║██║   ██║██╔════╝╚══██╔══╝██╔════╝\n"
         "███████╗██║   ██║██║   ██║██║        ██║   █████╗  \n"
         "╚════██║╚██╗ ██╔╝██║   ██║██║        ██║   ██╔══╝  \n"
         "███████║ ╚████╔╝ ╚██████╔╝╚██████╗   ██║   ██║     \n"
         "╚══════╝  ╚═══╝   ╚═════╝  ╚═════╝   ╚═╝   ╚═╝     \n"
         "                                                   \n"
         "         WELCOME TO SVUCTF HELLOWORLD 2023         \n"
         "---------------------------------------------------");
}

void gadget() {
    asm("pop %rdi; ret");
}

void b4ckdoor() {
    system("echo hi!");
}

void vuln() {
    int age;
    char buf[50];

    puts("But this time..you need to find the point to get it!");
    puts("What\'s your age?");
    scanf("%d", &age);
    puts("Now..try to overflow!");
    read(0, &buf, age);
}

int main() {
    init();
    banner();
    vuln();
    return 0;
}
