# 壳

- 作者：pn1fg
- 参考：-
- 难度：Normal
- 分类：Reverse
- 镜像：-
- 端口：-

## 题目描述

吃鸡蛋之前记得把壳敲碎哦！

## 题目解析

- 源码：[babyre.c](build/babyre.c)
- 考点：UPX 脱壳

题目中提示到了壳，这里给同学们解释一下程序加壳

**壳的概念**

壳是在一些计算机软件里也有一段专门负责保护软件不被非法修改或反编译的程序。 壳是在一个程序的外面再包裹上另外一段代码，保护里面的代码不被非法修改或反编译的程序。它们一般都是先于程序运行，拿到控制权，然后完成它们保护软件的任务。

**壳的分类：**

- 压缩壳
- UPX
- ASpack
- PECompat
- 加密壳
  - APSrotector
  - Armadillo
  - EXEXCryptor
  - Themida

#### 查看文件信息

查看文件类型，有无壳（`Exeinfo PE`）：

```
操作系统: Unix(0)[AMD64, 64 位, DYN]
打包工具: UPX(4.10)[NRV2B_LE32,best]
```

64 位程序，UPX壳

脱壳（UPX）：

```shell
$ upx -d babyre
                       Ultimate Packer for eXecutables
                          Copyright (C) 1996 - 2023
UPX 4.1.0       Markus Oberhumer, Laszlo Molnar & John Reiser    Aug 8th 2023

        File size         Ratio      Format      Name
   --------------------   ------   -----------   -----------
     24903 <-      5728   23.00%   linux/amd64   babyre

Unpacked 1 file.
```

反编译`gift`函数：

```c
void sym.gift(void)

{
    sym.imp.puts("MZWGCZ33IMYG4Z3SMF2HKMLBORUTA3TTL5UGCY3LMVZH2===");
    return;
}
```

这一串加密的字符串似乎就是 flag。

`Cyberchef`解密（\`base32）：

flag{C0ngratu1ati0ns_hacker}
