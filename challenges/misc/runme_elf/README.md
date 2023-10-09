# runeme_elf

- **作者**：13m0n4de
- **参考**：MoeCTF 2023
- **难度** ：1
- **分类**：Misc
- **暴露端口**：-

## 题目描述

下载文件，运行得 FLAG ~
诶诶出了点小问题，好像不能运行？！因为这个程序是 Linux 操作系统下的可执行文件。
请尝试配置一个 Linux 环境（虚拟机或者 WSL）来运行它。

## 题目解析

题目制作仿照 [MoeCTF 2023] Basic/runme2 。

- 附件：[runme](attachments/runme)

- 源码：[runme.c](build/runme.c)

配置好 Linux 环境（无论你是虚拟机还是 WSL 还是直接装在物理机上），将附件传入。

移动到同一目录，在终端中执行：

```shell
$ ./runme
```

如果你得到了无法执行之类的错误，说明从平台下载下来是不带执行权限的，需要添加权限后执行：

```shell
$ chmod +x runme
$ ./runme
flag{Run_m4_1n_l1nux!}
```

