# hard_nc

- 作者：pn1fg
- 参考：-
- 难度：Trivial
- 分类：Pwn
- 镜像：[svuctf-helloworld-2023/hard_nc](https://ghcr.io/svuctf/svuctf-helloworld-2023/hard_nc)
- 端口：70

# 题目描述

隐匿的 flag！

# 题目解析

本题也是考察 Pwn 的工具的使用，但更多的是考察`linux`操作系统的基本命令。

本题我们看到题目给的描述是`隐匿的flag！`，这就让我们联想到 linux 下的隐藏文件夹。

我们nc连接本题的容器后，依旧 ls 查看当前目录下的文件，发现没有flag文件。

```shell
nc 8.130.133.46 33988
ls
bin
dev
lib
lib32
lib64
```

我们再尝试去其它路径下查看，也都没有，这就应征了我们刚刚的猜想，**隐藏文件**

linux下查看隐藏文件：

- 命令：`ls -la`

```shell
ls -la
total 40
drwxr-x---    1 1000     1000          4096 Oct 14 10:22 .
drwxr-x---    1 1000     1000          4096 Oct 14 10:22 ..
drwxr-x---    1 1000     1000          4096 Oct 22 12:31 .secret
drwxr-xr-x    2 0        0             4096 Oct 14 10:22 bin
drwxr-xr-x    2 0        0             4096 Oct 14 10:22 dev
drwxr-xr-x    8 0        0             4096 Oct 14 10:22 lib
drwxr-xr-x    2 0        0             4096 Oct 14 10:22 lib32
drwxr-xr-x    2 0        0             4096 Oct 14 10:22 lib64
```

可以看到当前目录下有个`.secret`的隐藏文件夹，flag文件就在其中

获取 flag

```shell
cat .secret/f*
flag{6ff9a999-b773-4b55-97c2-62e5e28fa698}
```
