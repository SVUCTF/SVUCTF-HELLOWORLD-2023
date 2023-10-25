from pwn import *

context.arch = 'amd64'
context.log_level = 'debug'

if args['REMOTE']:
    io = remote('ip',port)
else:
    io = process('./GAME')

io.sendlineafter(b'Input:\n',os.popen('./random').read())
io.interactive()
