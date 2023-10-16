#include <stdio.h>
#include <string.h>

int main() {
    char buf[100];
    scanf("%99s", buf);

    int length = strlen(buf);
    int ptr_left = 0;
    int ptr_right = length - 1;

    while (ptr_right > ptr_left) {
        if (buf[ptr_left] != buf[ptr_right]) {
            printf("false");
            return 0;
        }
        ptr_left++;
        ptr_right--;
    }

    printf("true");
    return 0;
}

