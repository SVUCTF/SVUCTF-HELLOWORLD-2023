# Registrar

- 作者：pn1fg
- 参考：BUUCTF
- 难度：Trivial
- 分类：Reverse
- 镜像：-
- 端口：-

## 题目描述

-

## 题目解析

#### 查看文件信息

查看文件类型：

```shell
$ file Registrar.apk
Registrar.apk: Java archive data (JAR)
```

`apk`程序，安卓系统程序

反编译（JADX-GUI）：

这是个`Java`语言写的安卓 app ，反编译后我们直接查找重要关键字，定位到 flag

```java
         if (flag == 1) {
                    char[] x = "dd2940c04462b4dd7c450528835cca15".toCharArray();
                    x[2] = (char) ((x[2] + x[3]) - 50);
                    x[4] = (char) ((x[2] + x[5]) - 48);
                    x[30] = (char) ((x[31] + x[9]) - 48);
                    x[14] = (char) ((x[27] + x[28]) - 97);
                    for (int i = 0; i < 16; i++) {
                        char a = x[31 - i];
                        x[31 - i] = x[i];
                        x[i] = a;
                    }
                    String bbb = String.valueOf(x);
                    textview.setText("flag{" + bbb + "}");
```

flag 就是字符数组 x 中的一串字符，但是需要经过下面的一系列加密算法

#### 编写利用程序

```python
str=['d','d','2','9','4','0','c','0','4','4','6','2','b','4','d','d','7','c','4','5','0','5','2','8','8','3','5','c','c','a','1','5']

str[2] = chr(ord(str[2]) + ord(str[3]) - 50)
str[4] = chr(ord(str[2]) + ord(str[5]) - 48)
str[30] = chr(ord(str[31]) + ord(str[9]) - 48)
str[14] = chr(ord(str[27]) + ord(str[28]) - 97)

for i in range(16):
    s = str[31-i]
    str[31-i] = str[i]
    str[i] = s

for i in str:
    print (i,end="")
```
