#include <stdio.h>
#include <stdlib.h>
#include <time.h>


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

void secure() {
    int secretcode, input;
    srand(time(NULL));

    secretcode = rand();
    scanf("%d", &input);
    if (input == secretcode) {
        system("/bin/sh");
    }
}

void vuln() {
    char buf[100];
    puts("There is something amazing here, do you know anything?");
    gets(buf);
    puts("Maybe I will tell you next time !");
}

int main() {
    init();
    banner();
    vuln();
    return 0;
}


