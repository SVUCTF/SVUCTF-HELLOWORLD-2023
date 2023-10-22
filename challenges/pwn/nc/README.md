# 你会nc嘛？

- 作者：13m0n4de
- 参考：-
- 难度：Baby
- 分类：Pwn
- 暴露端口：70

# 题目描述

Netcat工具的使用

# 题目解析

本题考察Pwn方向的工具的使用，也是学Pwn的基础知识。

Netcat：

- 基本命令格式：nc 「ip address」 「port」

本题我们要做的就是`nc ip port`

```shell
nc 8.130.133.46 33986
```

连接成功后，有无回显的话是要看出题人在远程服务器上的部署，本题就属于没有回显

我们连上后是类似于一个shell的终端，出题人在部署题目的时候，会给一些linux的基本命令，比如`ls`，`cat`，`cd`等，我们可以直接`ls`查看当前目录下的文件，

```shell
ls
bin
dev
flag
lib
lib32
lib64
```

本题就是如此，`ls`就可以看到当前目录下有个`flag`文件

我们再用`cat`命令读取即可获得flag。

```shell
cat f*
flag{ecf047ff-3be8-4d80-a5a0-fe62524f492d}
```

