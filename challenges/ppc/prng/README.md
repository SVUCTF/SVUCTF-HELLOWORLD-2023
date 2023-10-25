# 绝对安全的随机数生成器

- 作者：13m0n4de
- 参考：-
- 难度：Medium
- 分类：PPC
- 暴露端口：70

## 题目描述

@zzz：「唉，这次比赛里 Pwn 题好多随机数都可以被预测，我都开始怀疑自己的程序安不安全了。」

@13m0n4de：「确实。」

@zzz：「怎么办呢？你说我用一个超长超复杂的随机种子会不会好一些？」

@zzz：「我懂了，再引入电脑风扇噪声、键盘敲击声音以及室友半夜的怪叫作为生成种子的因素...」

@zzz：「生成一千个随机字符，组合，变换，以地心生物的身份把字符作为垃圾邮件发给地球中潜藏的蜥蜴人，挑拨它们的关系...」

@zzz：「然后，趁乱黑掉天空中的全息投影，操控宇宙射线，把学某通中的作业提交截至时间修改到昨天...」

@zzz：「以班级里挂科的人数，再加上......」

@13m0n4de：「不是，你让新生来退出 Vim 不就好了，保证随机，完全不可预测。」

@zzz：「？」

> Hint：附件中引入的 freshman 文件与此题无关。（本题制作过程中没有任何一位新生受到伤害）

## 题目解析

这题是偏向 Crypto 的 PPC。

考点：梅森算法的破解，预测随机数。

题目附件 [main.py](attachments/main.py)：

```python
#!/usr/local/bin/python

import os
import random
import freshman


FLAG = os.getenv("GZCTF_FLAG", "fake{xxxxxxxx}")

seed = freshman.exit_vim() 
random.seed(seed)

while True:
    number = random.getrandbits(32)
    print("都说了我的种子万无一失，你没办法预测我的随机数")
    ctfer_input = input(f">> ")

    if int(ctfer_input) == number:
        print(FLAG)
        break
    else:
        print(f"猜错了喔，我输入的可是：")
        print(number)
```

### 关于梅森算法
