#include <stdio.h>
#include <sys/mman.h>

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
       "-----------------------------------------------------");

}

void vuln() {
  char *buf = mmap(0,0x400,7,34,0,0);
  puts("Please Send !!!!");
  read(0,buf,0x400);
  ((void(*)(void))buf)();
}

int main() {
  init();
  banner();
  vuln();
  return 0;
}
