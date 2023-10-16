#include <stdio.h>
#include <string.h>

int main() {
    int num;
    int result = 0;

    scanf("%d", &num);

    for (int i = 0; i <= num; i++) {
        if (i % 3 == 0 ||  i % 5 == 0 || i % 7 == 0) {
            result += i;
        }
    }
    printf("%d", result);

    return 0;
}

