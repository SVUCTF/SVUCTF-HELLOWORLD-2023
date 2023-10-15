from pwn import *
from randcrack import RandCrack

# context.log_level = "DEBUG"

rc = RandCrack()

io = remote("127.0.0.1", 8887)

for _ in range(624):
    io.sendlineafter(b">> ", b"0")
    io.recvline()
    recv_data = io.recvline(keepends=False).decode()
    number = int(recv_data)
    rc.submit(number)

next_number = rc.predict_getrandbits(32)

success(next_number)

io.sendlineafter(b">> ", str(next_number).encode())
io.interactive()
