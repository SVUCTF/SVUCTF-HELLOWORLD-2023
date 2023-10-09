#include <stdio.h>

int main() {
  unsigned char enc_data[21] = "fmcd\x7FMGOIVIT=~QJ$bk2i";
  unsigned char flag[21];
  for (int i = 0; i < 21; i++) {
    flag[i] = enc_data[i] ^ i;
  }
  printf("%s\n", flag);
  return 0;
}
