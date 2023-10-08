# runeme_python

- **作者**：13m0n4de
- **参考**：MoeCTF 2023
- **难度** ：1
- **分类**：Misc
- **暴露端口**：-

## 题目描述

todo!

## 题目解析

```python
flag = "flag{Pyth0n_1z_0ur_g00d_friendz}"

enc2 = [((0xF3) & ord(i) | (ord(i) ^ 0xFF) & 0xC) for i in flag]
enc1 = [i ^ 0xFF for i in enc2]
print(enc1)
```

