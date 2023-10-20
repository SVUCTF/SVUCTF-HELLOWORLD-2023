#include <stdio.h>

void flag_checker(int v, int w, int x, int y, int z) {
    if ((v * 24 + w * -32 + x * 98 + y * 55 + z * 90 == 153908) &&
        (v * 123 + w * -332 + x * 68 + y * 67 + z * 32 == -63551) &&
        (v * 266 + w * -34 + x * 44 + y * 8 + z * 32 == 190984) &&
        (v * 454 + w * -302 + x * 51 + y * 65 + z * 5 == 124630) &&
        (v * 321 + w * -321 + x * 938 + y * 565 + z * 970 == 1543640)) {
        puts("Congratulations, Here is your flag:");
        printf("flag{%d_%d_%d_%d_%d}\n", v, w, x, y, z);
    } 
    else {
        puts("\nFailed\n");
    }
}

int main() {
    int v, w, x, y, z;
    printf("Input 5 random numbers and check your luck ;)\n");
    printf("V: ");
    scanf("%d", &v);
    printf("W: ");
    scanf("%d", &w);
    printf("X: ");
    scanf("%d", &x);
    printf("Y: ");
    scanf("%d", &y);
    printf("Z: ");
    scanf("%d", &z);
    printf("\n");
    flag_checker(v, w, x, y, z);
    return 0;
}


