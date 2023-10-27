# 米哈游

- 作者：ksks
- 参考：-
- 难度：Easy
- 分类：Web
- 镜像：[svuctf-helloworld-2023/md5](https://ghcr.io/svuctf/svuctf-helloworld-2023/md5)
- 端口：80

## 题目描述

我因为启动了原神获得了测试他们的网站的资格，然后我往里边放了一个flag，快来看看这个网站的flag在哪里把

## 题目解析

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
            background-color: #f2f2f2;
        }
        header {
            background-color: #333;
            color: #fff;
            text-align: center;
            padding: 20px;
        }
        main {
            max-width: 800px;
            margin: 20px auto;
            background-color: #fff;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }
        img {
            max-width: 100%;
            height: auto;
        }
    </style>
</head>
<body>
<header>
    <h1></h1>
</header>
<main>
    <article>
        <h2>这是一个收藏miHOYO表情包的站点</h2>
        <p></p>
        <p></p>
        <img src="x.png" alt="图片描述">
        <img src="x1.png" alt="图片描述">
        <img src="x2.png" alt="图片描述">
        <img src="x3.png" alt="图片描述">
        <img src="x4.png" alt="图片描述">
        <img src="x5.png" alt="图片描述">
        <img src="x6.png" alt="图片描述">
    </article>

</main>

<div class="bottom-button">
    <a href="md5.php" class="button">点我进一步查看更多表情包</a>
</div>

</body>
</html>
```

```php
<?php
highlight_file(__FILE__);
#哈哈哈被骗了吧
#完成这道md5我就给你flag
if (isset($_GET['name']) && isset($_POST['password'])) {
    $name = $_GET['name'];
    $password = $_POST['password'];
    if ($name != $password && md5($name) == md5($password)) {
        include 'flag2.php';
    } else {
        echo "上网搜搜，就差一点点啦！！！";
    }
} else {
    echo '来吧来吧，快写，我才会给你米哈游的表情包';
}
```

flag文件

```php
<?php
$flag = 'flag{xxxxxxxxxxxx}';
echo $flag;

?>
```

考点：md5的弱比较
