# runeme_c

- **作者**：13m0n4de
- **参考**：MoeCTF 2023
- **难度** ：1
- **分类**：Misc
- **暴露端口**：-

## 题目描述

C 语言是学习计算机基础中的基础，也是计算机第一学期的必修课。本题你需要配置一个能够编译运行 C 语言程序的环境，并且运行题目给出的代码来获取 FLAG 。

## 题目解析

题目制作仿照 [MoeCTF 2023] Basic/CCCCC 。

附件如下：

```c
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
```

使用任何编译工具将 C 语言源文件编译成可执行文件，执行后即可得到 FLAG。

举一个命令行中使用 GCC 编译的例子，你使用任何 IDE 之类的工具编译运行也是可以的。

```shell
$ gcc runme.c -o runme
$ ./runme
flag{HAHA_C_1s_E4sy!}
```

当然，你也可以试试读懂这段代码。