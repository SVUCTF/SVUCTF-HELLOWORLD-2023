# SVUCTF-HELLOWORLD-2023

本仓库用于存储和构建 SVUCTF-HELLOWORLD-2023 的题目镜像。

Powered by GZCTF and GZTime

## 目录

项目结构遵循 GZCTF 规范。

```
.github/workflows/                      # github actions
    └── chal.<category>.<name>.yml      # 每个题目的编译脚本
challenges/                             # 题目目录
    ├── challenge1/
    │   ├── build/                      # 题目构建目录
    │   │   ├── Dockerfile
    │   │   └── more...
    │   ├── attachments/                # 题目附件目录
    │   ├── writeup/                    # 题目题解目录    
    │   └── README.md
    ├── challenge2/
    └── more...
```

### Misc

| 题目描述与题解                                              | 难度    | 文件、源代码                                             | 镜像 | 出题人       |
|:------------------------------------------------------------|:--------|:---------------------------------------------------|:-----|:-------------|
| [runme_c](challenges/misc/runme_c/README.md)                | Baby    | [文件、源代码](challenges/misc/runme_c/attachments)      | -    | 13m0n4de     |
| [runeme_python](challenges/misc/runme_python/README.md)     | Baby    | [文件、源代码](challenges/misc/runme_python/attachments) | -    | 13m0n4de     |
| [runeme_elf](challenges/misc/runme_elf/README.md)           | Baby    | [文件、源代码](challenges/misc/runme_elf/build)          | -    | 13m0n4de     |
| [runeme_exe](challenges/misc/runme_exe/README.md)           | Baby    | [文件、源代码](challenges/misc/runme_exe/build)          | -    | 13m0n4de     |
| [碰撞！碰撞！碰撞！](challenges/misc/peng/README.md)           | Baby    | [文件、源代码](challenges/misc/peng/attachments)         | -    | Wh1te_0range |
| [很可爱的一只猫](challenges/misc/cat/README.md)             | Baby    | [文件、源代码](challenges/misc/cat/attachments)          | -    | Wh1te_0range |
| [各自离开后的终章](challenges/misc/love/README.md)          | Baby    | [文件、源代码](challenges/misc/love/attachments)         | -    | Wh1te_0range |
| [夜晚的星](challenges/misc/evening/README.md)               | Trivial | [文件、源代码](challenges/misc/evening/attachments)      | -    | Wh1te_0range |
| [Easy社工](challenges/misc/easy_osint/README.md)            | Trivial | [文件、源代码](challenges/misc/easy_osint/attachments)   | -    | Wh1te_0range |
| [原神！重启！](challenges/misc/yuanshen/README.md)            | Trivial | [文件、源代码](challenges/misc/yuanshen/attachments)     | -    | Wh1te_0range |
| [SharkShark!芜湖！](challenges/misc/shark/README.md)         | Easy    | [文件、源代码](challenges/misc/shark/attachments)        | -    | Wh1te_0range |
| [可曾听闻俄罗斯套娃？](challenges/misc/tw/README.md)         | Easy    | [文件、源代码](challenges/misc/tw/attachments)           | -    | Wh1te_0range |
| [Baby社工](challenges/misc/baby_osint/README.md)            | Normal  | [文件、源代码](challenges/misc/baby_osint/attachments)   | -    | Wh1te_0range |
| [救命我被 Vim 困住了](challenges/misc/vim_escape/README.md) | Medium  | [文件、源代码](challenges/misc/vim_escape/build)         | -    | 13m0n4de     |

Misc 有一些题目没有放出，大多是因为考点重复或者太简单，但还是放在仓库里了，它们没有题解。

| 题目描述与题解                                             | 难度 | 文件、源代码                                         | 镜像 | 出题人       |
|:-------------------------------------------------------|:-----|:-----------------------------------------------|:-----|:-------------|
| [破碎的铠甲撒满了鲜花](challenges/misc/kaisa/README.md)    | -    | [文件、源代码](challenges/misc/kaisa/attachments)    | -    | Wh1te_0range |
| [从前有个人叫贺希](challenges/misc/hex/README.md)          | -    | [文件、源代码](challenges/misc/hex/attachments)      | -    | Wh1te_0range |
| [Base家族来了](challenges/misc/base/README.md)             | -    | [文件、源代码](challenges/misc/base/attachments)     | -    | Wh1te_0range |
| [仔细看看这是什么](challenges/misc/file/README.md)         | -    | [文件、源代码](challenges/misc/file/attachments)     | -    | Wh1te_0range |
| [摇起来！嗷~](challenges/misc/base2/README.md)              | -    | [文件、源代码](challenges/misc/base2/attachments)    | -    | Wh1te_0range |
| [猫咪和鲸鱼的巧妙结合](challenges/misc/cat_fish/README.md) | -    | [文件、源代码](challenges/misc/cat_fish/attachments) | -    | Wh1te_0range |

### Reverse

| 题目描述与题解                                      | 难度    | 文件、源代码                                             | 镜像 | 出题人 |
|:----------------------------------------------------|:--------|:---------------------------------------------------|:-----|:-------|
| [gift](challenges/reverse/gift/README.md)           | Baby    | [文件、源代码](challenges/reverse/gift/build)            | -    | pn1fg  |
| [Registrar](challenges/reverse/Registrar/README.md) | Trivial | [文件、源代码](challenges/reverse/Registrar/attachments) | -    | pn1fg  |
| [python](challenges/reverse/python/README.md)       | Normal  | [文件、源代码](challenges/reverse/python/build)          | -    | pn1fg  |
| [壳](challenges/reverse/UPX/README.md)              | Normal  | [文件、源代码](challenges/reverse/UPX/build)             | -    | pn1fg  |
| [方程](challenges/reverse/z3/README.md)             | Normal  | [文件、源代码](challenges/reverse/z3/build)              | -    | pn1fg  |

### Pwn

| 题目描述与题解                                        | 难度    | 文件、源代码                                     | 镜像                                                                                            | 出题人   |
|:------------------------------------------------------|:--------|:-------------------------------------------|:------------------------------------------------------------------------------------------------|:---------|
| [你会nc嘛？](challenges/pwn/nc/README.md)              | Baby    | [文件、源代码](challenges/pwn/nc/build)          | [svuctf-helloworld-2023/nc](https://ghcr.io/svuctf/svuctf-helloworld-2023/nc)                   | 13m0n4de |
| [hard_nc](challenges/pwn/hard_nc/README.md)           | Trivial | [文件、源代码](challenges/pwn/hard_nc/build)     | [svuctf-helloworld-2023/hard_nc](https://ghcr.io/svuctf/svuctf-helloworld-2023/hard_nc)         | pn1fg    |
| [没有权限的flag！](challenges/pwn/SUID/README.md)      | Easy    | [文件、源代码](challenges/pwn/SUID/build)        | [svuctf-helloworld-2023/suid](https://ghcr.io/svuctf/svuctf-helloworld-2023/suid)               | pn1fg    |
| [ret2text_32](challenges/pwn/ret2text_32/README.md)   | Normal  | [文件、源代码](challenges/pwn/ret2text_32/build) | [svuctf-helloworld-2023/ret2text_32](https://ghcr.io/svuctf/svuctf-helloworld-2023/ret2text_32) | pn1fg    |
| [GAME](challenges/pwn/GAME/README.md)                 | Normal  | [文件、源代码](challenges/pwn/GAME/build)        | [svuctf-helloworld-2023/game](https://ghcr.io/svuctf/svuctf-helloworld-2023/game)               | 13m0n4de |
| [简单的shellcode](challenges/pwn/shellcode/README.md) | Normal  | [文件、源代码](challenges/pwn/shellcode/build)   | [svuctf-helloworld-2023/shellcode](https://ghcr.io/svuctf/svuctf-helloworld-2023/shellcode)     | pn1fg    |
| [ret2text_64](challenges/pwn/ret2text_64/README.md)   | Medium  | [文件、源代码](challenges/pwn/ret2text_64/build) | [svuctf-helloworld-2023/ret2text_64](https://ghcr.io/svuctf/svuctf-helloworld-2023/ret2text_64) | pn1fg    |
| [ROP链](challenges/pwn/ROP/README.md)                 | Medium  | [文件、源代码](challenges/pwn/ROP/build)         | [svuctf-helloworld-2023/rop](https://ghcr.io/svuctf/svuctf-helloworld-2023/rop)                 | pn1fg    |

### PPC

| 题目描述与题解                                                        | 难度   | 文件、源代码                                      | 镜像                                                                                              | 出题人   |
|:-------------------------------------------------------------------|:-------|:--------------------------------------------|:--------------------------------------------------------------------------------------------------|:---------|
| [高 Ping 战士](challenges/ppc/pwntools/README.md)                     | Easy   | [文件、源代码](challenges/ppc/pwntools/build)     | [svuctf-helloworld-2023/pwntools](https://ghcr.io/svuctf/svuctf-helloworld-2023/pwntools)         | 13m0n4de |
| [我们怀疑您是人类，请完成机器人验证](challenges/ppc/captcha/README.md) | Easy   | [文件、源代码](challenges/ppc/captcha/build)      | [svuctf-helloworld-2023/captcha](https://ghcr.io/svuctf/svuctf-helloworld-2023/captcha)           | 13m0n4de |
| [绝对安全的随机数生成器](challenges/ppc/prng/README.md)               | Medium | [文件、源代码](challenges/ppc/prng/build)         | [svuctf-helloworld-2023/prng](https://ghcr.io/svuctf/svuctf-helloworld-2023/prng)                 | 13m0n4de |
| [更适合破晓宝宝的 OJ](challenges/ppc/online_judge/README.md)          | Medium | [文件、源代码](challenges/ppc/online_judge/build) | [svuctf-helloworld-2023/online_judge](https://ghcr.io/svuctf/svuctf-helloworld-2023/online_judge) | 13m0n4de |

### Crypto

| 题目描述与题解                                         | 难度    | 文件、源代码                                           | 镜像 | 出题人 |
|:-------------------------------------------------------|:--------|:-------------------------------------------------|:-----|:-------|
| [中国特色社会主义！](challenges/crypto/CHain/README.md) | Baby    | [文件、源代码](challenges/crypto/CHain/attachments)    | -    | Only   |
| [BASE！](challenges/crypto/BASE/README.md)              | Baby    | [文件、源代码](challenges/crypto/BASE/attachments)     | -    | Only   |
| [转转](challenges/crypto/rot13/README.md)              | Trivial | [文件、源代码](challenges/crypto/rot13/attachments)    | -    | Only   |
| [好喜欢表情](challenges/crypto/EMO/README.md)          | Trivial | [文件、源代码](challenges/crypto/EMO/attachments)      | -    | Only   |
| [Vigenere](challenges/crypto/Vigenere/README.md)       | Easy    | [文件、源代码](challenges/crypto/Vigenere/attachments) | -    | Only   |
| [佛曰：你悟了没？](challenges/crypto/fo/README.md)       | Easy    | [文件、源代码](challenges/crypto/fo/attachments)       | -    | Only   |
| [到底有几种编码啊？](challenges/crypto/code/README.md)  | Normal  | [文件、源代码](challenges/crypto/code/attachments)     | -    | Only   |
| [Ez-RSA](challenges/crypto/RSA/README.md)              | Normal  | [文件、源代码](challenges/crypto/RSA/attachments)      | -    | Only   |
| [Baby-RSA](challenges/crypto/baby_rsa/README.md)       | Medium  | [文件、源代码](challenges/crypto/baby_rsa/attachments) | -    | Only   |

与 Misc 一样，一些题目没有放出，因为考点重复或者太简单，但还是放在仓库里了，它们没有题解。

| 题目描述与题解                           | 难度 | 文件、源代码                                       | 镜像 | 出题人 |
|:-----------------------------------------|:-----|:---------------------------------------------|:-----|:-------|
| [ROMA](challenges/crypto/ROMA/README.md) | Baby | [文件、源代码](challenges/crypto/ROMA/attachments) | -    | Only   |

### Web

| 题目描述与题解                                                     | 难度    | 文件、源代码                                              | 镜像                                                                                                              | 出题人   |
|:-------------------------------------------------------------------|:--------|:----------------------------------------------------|:------------------------------------------------------------------------------------------------------------------|:---------|
| [今日没课，睡个觉吧](challenges/web/view_source/README.md)          | Baby    | [文件、源代码](challenges/web/view_source/build)          | [svuctf-helloworld-2023/view_source](https://ghcr.io/svuctf/svuctf-helloworld-2023/view_source)                   | Only     |
| [艺术就是爆炸](challenges/web/crack/README.md)                     | Trivial | [文件、源代码](challenges/web/crack/build)                | [svuctf-helloworld-2023/crack](https://ghcr.io/svuctf/svuctf-helloworld-2023/crack)                               | ksks     |
| [GET](challenges/web/get/README.md)                                | Trivial | [文件、源代码](challenges/web/get/build)                  | [svuctf-helloworld-2023/get](https://ghcr.io/svuctf/svuctf-helloworld-2023/get)                                   | ksks     |
| [中场休息玩个游戏](challenges/web/missile_trail/README.md)         | Easy    | [文件、源代码](challenges/web/missile_trail/build)        | [svuctf-helloworld-2023/missile_trail](https://ghcr.io/svuctf/svuctf-helloworld-2023/missile_trail)               | Only     |
| [原神，启动！](challenges/web/non_pressable_button/README.md)        | Easy    | [文件、源代码](challenges/web/non_pressable_button/build) | [svuctf-helloworld-2023/non_pressable_button](https://ghcr.io/svuctf/svuctf-helloworld-2023/non_pressable_button) | ksks     |
| [米哈游](challenges/web/md5/README.md)                             | Easy    | [文件、源代码](challenges/web/md5/build)                  | [svuctf-helloworld-2023/md5](https://ghcr.io/svuctf/svuctf-helloworld-2023/md5)                                   | ksks     |
| [更适合破晓宝宝的代码编辑器](challenges/web/code_editor/README.md) | Easy    | [文件、源代码](challenges/web/code_editor/build)          | [svuctf-helloworld-2023/code_editor](https://ghcr.io/svuctf/svuctf-helloworld-2023/code_editor)                   | 13m0n4de |

## 难度与分值

| 题目难度         | Baby | Trivial | Easy | Normal | Medium |
|:-------------|------|---------|------|--------|--------|
| 题目分值         | 100  | 200     | 400  | 800    | 1200   |
| 题目最低分值比例 | 50%  | 80%     | 80%  | 80%    | 80%    |
| 题目最低分值     | 50   | 160     | 320  | 640    | 960    |
| 难度系数         | 3.0  | 4.0     | 5.0  | 6.0    | 7.0    |

## 来自参赛者的题解

## 致谢

- [W4terCTF-2023](https://github.com/W4terDr0p/W4terCTF-2023)：作为项目目录结构以及工作流文件编写的参考
- [GZCTF](https://github.com/GZTimeWalker/GZCTF/)：比赛平台
