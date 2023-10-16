<?php
highlight_file(__FILE__);
include 'flag.php';

# 哈哈哈被骗了吧
# 完成这道 MD5 我就给你flag
if (isset($_GET['name']) && isset($_POST['password'])) {
    $name = $_GET['name'];
    $password = $_POST['password'];
    if ($name != $password && md5($name) == md5($password)) {
        echo $flag;
    } else {
        echo "上网搜搜，就差一点点啦！！！";
    }
} else {
    echo '来吧来吧，快写，我才会给你米哈游的表情包';
}
