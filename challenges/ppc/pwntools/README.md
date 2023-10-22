# 高 Ping 战士

- 作者：13m0n4de
- 参考：-
- 难度：Easy
- 分类：PPC
- 暴露端口：70

## 题目描述

@zzz：*打游戏中....*

@ksks：「在干嘛？打电动呢？」

@zzz：「...」

@13m0n4de：「你这也太卡了，人物都成了慢动作。」

@zzz：「...」

@ksks：「别玩了，延迟图标都红了，还打得到人吗？」

@zzz：「...」

@ksks：「你看，死了吧，等打出子弹，别人都杀你十回了。」

@zzz：「...」

@13m0n4de：「你要不还是去玩石头剪刀布吧。」

@zzz：「对，我在打电动。」

## 题目解析

考点：Pwntools 基本使用

@zzz 同学的延迟实在是太高了，石头剪刀布竟然没法保证双方同时伸手，那不是肯定会输嘛。

与测试赛时的 `baby_calculator` 一致，这题是根据返回内容发送对应数据的题目，重复 500 次。

[solve.py](writeup/solve.py)

```python
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
```

*IE 笑话和 IE 一样过时了*