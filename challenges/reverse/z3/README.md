# 方程

- 作者：pn1fg
- 参考：-
- 难度：Normal
- 分类：Reverse
- 镜像：-
- 端口：-

## 题目描述

你会解五元方程组嘛？

## 题目解析

#### 查看文件信息

查看文件类型：

```shell
$ file Equation
Equation: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=0e39510b411a93871d093f8227631103410274c0, for GNU/Linux 4.4.0, not stripped
```

查壳：

```
操作系统: Linux(ABI: 4.4.0)[AMD64, 64 位, DYN]
编译器: GCC(13.2.1 20230801)
语言: C/C++
```

无壳

反编译`flag_checker`函数：

```c
void sym.flag_checker(ulong arg1, ulong arg2, ulong arg3, ulong arg4, ulong arg5)

{
    ulong var_4h;

    if ((((arg5 * 0x5a + arg1 * 0x18 + arg2 * -0x20 + arg3 * 0x62 + arg4 * 0x37 == 0x25934) &&
         (arg5 * 0x20 + arg1 * 0x7b + arg2 * -0x14c + arg3 * 0x44 + arg4 * 0x43 == -0xf83f)) &&
        (arg5 * 0x20 + arg1 * 0x10a + arg2 * -0x22 + arg3 * 0x2c + arg4 * 8 == 0x2ea08)) &&
       ((arg5 * 5 + arg1 * 0x1c6 + arg2 * -0x12e + arg3 * 0x33 + arg4 * 0x41 == 0x1e6d6 &&
        (arg5 * 0x3ca + arg1 * 0x141 + arg2 * -0x141 + arg3 * 0x3aa + arg4 * 0x235 == 0x178dd8)))) {
        sym.imp.puts("Congratulations, Here is your flag:");
        sym.imp.printf("flag{%d_%d_%d_%d_%d}\n", arg1, arg2, arg3, arg4, arg5);
        return;
    }
    sym.imp.puts("\nFAILED!\n");
    return;
}
```

分析程序，这是个类似于解方程的程序，刷题量多的同学可以看出这个其实在考察`z3约束求解`，本题的程序只是其中最基础的

#### 编写利用程序

```python
from z3 import *

v = Real('v')
w = Real('w')
x = Real('x')
y = Real('y')
z = Real('z')

s = Solver()

s.add(v * 24 + w * -32 + x * 98 + y * 55 + z * 90 == 153908)
s.add(v * 123 + w * -332 + x * 68 + y * 67 + z * 32 == -63551)
s.add(v * 266 + w * -34 + x * 44 + y * 8 + z * 32 == 190984)
s.add(v * 454 + w * -302 + x * 51 + y * 65 + z * 5 == 124630)
s.add(v * 321 + w * -321 + x * 938 + y * 565 + z * 970 == 1543640)

if s.check() == sat:
    result = s.model()
    print (result)
else:
    print (b'no result')
```

本题的初衷是要让大家了解`z3约束求解`，好多同学都用了数学解方程的思想解出了题目，所以用在线网站解方程的求解过程就不展示了
