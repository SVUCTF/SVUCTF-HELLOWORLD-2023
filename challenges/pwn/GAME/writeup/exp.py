from pwn import *
from ctypes import cdll

context.arch = 'amd64'
context.log_level = 'debug'

if args['REMOTE']:
    io = remote('8.130.133.46',34036)
else:
    io = process('./GAME')

libc = cdll.LoadLibrary("libc.so.6")
libc.srand(libc.time(0))
number = libc.rand() % 999 + 1

io.sendlineafter(b'Input:\n',str(number).encode())
io.interactive()
