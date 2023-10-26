# gift

- 作者：pn1fg
- 参考：-
- 难度：Baby
- 分类：Reverse
- 镜像：-
- 端口：-

## 题目描述

@YYYY：你知道 ELF 反汇编嘛？

@ZZZZ：并不知道。

@YYYY：我也不知道，长大后再学吧！

## 题目解析

- 源码：[easyre.c](build/easyre.c)
- 考点：ELF 反汇编

查看文件类型：

```shell
$ file easyre
easyre: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=4be7e5e0b76d17d29e5b7bb039ae4f9bae4fd5c8, for GNU/Linux 4.4.0, not stripped
```

64 位 ELF 文件

反编译`gift`函数

```c
void sym.gift(void)

{
    sym.imp.puts("ZmxhZ3tXZWxjMG1lX3QwX1NWVUNURiF9");
    return;
}
```

得到字符串，字符串套了一层 base64

flag：`flag{Welc0me_t0_SVUCTF!}`

