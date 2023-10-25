# runeme_exe

- 作者：13m0n4de
- 参考：MoeCTF 2023
- 难度：Baby
- 分类：Misc
- 暴露端口：-

## 题目描述

下载文件，双击运行得 FLAG ~
但是我的程序好像会闪退欸，能不能想个办法保留一下它的输出？比如用 CMD 来运行它试试？
如果你不知道什么是 CMD，可以尝试使用搜索引擎来学习，加油吧 (> \<)

## 题目解析

题目制作仿照 \[MoeCTF 2023\] Basic/runme 。

附件：[runme.exe](attachments/runme.exe)

源码：[runme.c](build/runme.c)

双击执行 `runme.exe` ，出现一个一闪而过的黑框框，不要害怕自己运行了什么奇怪的病毒。

但是，**不要在本机上运行任何来源不明，功能不清的程序**是网络安全的第一课，哪怕是比赛的附件。

如果你点击下载，直接运行，只能感谢你十分信任我们啦。

这里出现的一闪而过的黑框中，其实输出了 FLAG ，但是由于输出后程序结束，窗口立刻就关闭了。

根据题目描述中所说，在 CMD 中运行，可以一直留下这个框框，也就能记下 FLAG 了。

```
.\runme.exe
```

如果你使用 Windows 10/11 ，你有更好看更好用的终端应用 [Windows Terminal](https://apps.microsoft.com/store/detail/windows-terminal/9N0DX20HK701) 可选。（它只是终端，与 CMD 和 PowerShell 是两类东西）
