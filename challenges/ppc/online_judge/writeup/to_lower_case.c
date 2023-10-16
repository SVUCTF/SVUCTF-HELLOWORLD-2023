#include <stdio.h>
#include <string.h>

int main() {
    char s[100];

    fgets(s, sizeof(s), stdin);

    int len = strlen(s);
    for (int i = 0; i < len; ++i) {
        if (s[i] >= 65 && s[i] <= 90) {
            s[i] |= 32;
        }
    }
    printf("%s", s);
    return 0;
}

