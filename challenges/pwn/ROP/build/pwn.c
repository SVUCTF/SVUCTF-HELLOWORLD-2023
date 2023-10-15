#include <stdio.h>
#include <stdlib.h>

char secret[7] = "/bin/sh";

void init() {
  setvbuf(stdout, 0, 2, 0);
  setvbuf(stdin, 0, 2, 0);
  setvbuf(stderr, 0, 2, 0);
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
  asm("pop %rdi;ret;");
}

void vuln() {
  char buf[100];
  system("echo You know the size of the input data?");
  gets(buf);
}

int main() {
  init();
  banner();
  vuln();
  return 0;
}
