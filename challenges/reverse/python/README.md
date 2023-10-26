# python

- 作者：pn1fg
- 参考：-
- 难度：Normal
- 分类：Reverse
- 端口：-

## 题目描述

你知道py反编译嘛？

## 题目解析
查看文件类型：

```shell
$ file hardre.pyc
hardre.pyc: python 3.6 byte-compiled
```

根据题目提示 python 反编译（`pycdc` pyc反编译工具）

```python
$ pycdc hardre.pyc
# Source Generated with Decompyle++
# File: hardre.pyc (Python 3.6)

import base64

def encode(message):
    s = ''
    for i in message:
        x = ord(i) - 5
        s += chr(x)

    return base64.b64encode(s.encode())

correct = 'YWdcYnZUanBaXG1gWnJkaXg='
flag = ''
print('Please Input flag:')
flag = input()
if encode(flag) == correct:
    print('Win!')
else:
    print('Failed!')
```

程序执行流程：输入 flag --> 遍历字符串每个字符的 ASCII 值减 5 --> Base64加密 --> 字符串比较

编写利用程序：

```python
import base64

correct = "YWdcYnZUanBaXG1gWnJkaXg="

message = base64.b64decode(correct).decode()

flag = ""
for i in message:
    x = ord(i) + 5
    flag += chr(x)

print(flag)
```

 
