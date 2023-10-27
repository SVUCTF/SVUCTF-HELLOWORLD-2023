# Ez-RSA

- 作者：Only
- 参考：-
- 难度：Normal
- 分类：Crypto
- 镜像：-
- 端口：-

## 题目描述

RSA？那是什么？百度一下吧

## 题目解析

```python
#!/usr/bin/env python
import gmpy2
from Crypto.Util.number import *
from binascii import a2b_hex, b2a_hex
flag = '******************'
p = 0xED7FCFABD3C81C78E212323329DC1EE2BEB6945AB29AB51B9E3A2F9D8B0A22101E467
q = 0xAD85852F9964DA87880E48ADA5C4487480AA4023A4DE2C0321C170AD801C9
e = 65537
n = p * q
c = pow(int(b2a_hex(flag), 16), e, n)
print(c)
c = 0x75AB3202DE3E103B03C680F2BEBBD1EA689C8BF260963FE347B3533B99FB391F0A358FFAE5160D6DCB9FCD75CD3E46B2FE3CFFE9FA2E9508702FD6E4CE43486631
# 友情提示：解出来得到明文后需要进行一次base64哦！
```

这里的RSA是最基础的 只要稍微了解一下RSA的加密原理就能解出来

EXP:

```python
import base64
import gmpy2
from Crypto.Util.number import long_to_bytes

p = 0xED7FCFABD3C81C78E212323329DC1EE2BEB6945AB29AB51B9E3A2F9D8B0A22101E467
q = 0xAD85852F9964DA87880E48ADA5C4487480AA4023A4DE2C0321C170AD801C9
e = 65537
c = 0x75AB3202DE3E103B03C680F2BEBBD1EA689C8BF260963FE347B3533B99FB391F0A358FFAE5160D6DCB9FCD75CD3E46B2FE3CFFE9FA2E9508702FD6E4CE43486631

n = q * p
d = gmpy2.invert(e, (p - 1) * (q - 1))
print("d=", d)
m = pow(c, d, n)
flag = long_to_bytes(m)
flag = base64.b64decode(flag)
print(flag.decode())
```

`flag{IlikeCTFbutCTFdon'tlikeme}`
