from pwn import *

io = remote("127.0.0.1", 8887)

for _ in range(500):
    io.recvuntil("@zzz 同学好像要出 ".encode())
    zzz_input = io.recvline(keepends=False).decode()
    match zzz_input:
        case "石头":
            my_input = "布"
        case "剪刀":
            my_input = "石头"
        case "布":
            my_input = "剪刀"
        case _:
            print(zzz_input)
            exit()

    io.sendlineafter("选择石头、剪刀或布：".encode(), my_input.encode())

io.interactive()

