# runeme_python

- **作者**：13m0n4de
- **参考**：MoeCTF 2023
- **难度**：1
- **分类**：Misc
- **暴露端口**：-

## 题目描述

Python 是 CTF 中最常用的编程语言，不管是学习哪个方向都离不开 Python。本题你需要配置一个能够解释运行 Python 程序的环境，并且运行题目给出的代码来获取 FLAG 。

## 题目解析

题目制作仿照 [MoeCTF 2023] Basic/Python 。

附件代码：

```python
enc1 = [149, 159, 146, 148, 136, 163, 138, 135, 155, 195, 157, 172, 194, 137, 172, 195, 134, 129, 172, 148, 195, 195, 151, 172, 149, 129, 154, 150, 157, 151, 137, 142]
enc2 = [i ^ 0xFF for i in enc1]
flag = [chr((0xF3 & i) | (i ^ 0xFF & 0xC)) for i in enc2]
flag = "".join(flag)
print(flag)
```

在 [Python 官网](https://www.python.org/downloads/)下载安装与你操作系统对应的 Python 解释器（此题无版本要求，日常使用的话推荐最新的稳定版），之后运行附件文件即可。

和上题 [runme_c](../runme_c/README.md) 一样，如果你用了 PyCharm 等 IDE 帮你完成了这些工作，也 OK 。

```shell
$ python runme.py
flag{Pyth0n_1z_0ur_g00d_friendz}
```

如果你好奇 `enc1` 是怎么得出来的（或者说这题怎么出的），下面是出题的代码；

```python
flag = "flag{Pyth0n_1z_0ur_g00d_friendz}"

enc2 = [((0xF3) & ord(i) | (ord(i) ^ 0xFF) & 0xC) for i in flag]
enc1 = [i ^ 0xFF for i in enc2]
print(enc1)
```

