<?php

if (isset($_POST["action"]) && $_POST["action"] === "加入原趴") {
    header("Location: /startup/index.php");
}
?>

<html>

<head>
    <title>Welcome to SVUCTF!!!</title>
    <style>
        body {
            font-family: "Arial", sans-serif;
            background-color: #65d2c6;
            margin: 0;
            padding: 0;
        }

        header {
            background-color: rgb(4, 168, 129);
            color: #fbfbfc;
            text-align: center;
            padding: 20px;
        }

        h1 {
            font-size: 36px;
        }

        .container {
            max-width: 800px;
            margin: 20px auto;
            background-color: #fcfcfd;
            padding: 20px;
            box-shadow: 0 0 10px rgb(80, 91, 120);
            border-radius: 10px;
        }

        p {
            font-size: 18px;
            line-height: 1.6;
        }

        .button {
            background-color: #f9cb09;
            color: white;
            border: none;
            padding: 10px 20px;
            font-size: 20px;
            border-radius: 5px;
            cursor: pointer;
            text-decoration: none;
        }

        .button:hover {
            background-color: #1d1e1f;
        }
    </style>
</head>

<body>
    <header>
        <h1>Welcome to SVUCTF!!!</h1>
    </header>
    <div class="container">
        <h2>按下按钮就可以启动原神</h2>
        <p>
            玩原神之前我是一个胆小又卑微得男孩子(◞‸◟)
        </p>
        <p>
            玩原神之后，我是一个热爱原神、充满爱心和幸福的人！我的世界充满了甜蜜和快乐(〃∀〃)
        </p>
        <p>
            你也想要跟我一样吗，那就按下按钮启动原神吧 (<ゝω・) 綺羅星☆ 想要前往提瓦特大陆那就按下按钮加入原趴
        </p>
        <h3>快按我！！快！！！我要玩原神</h3>
        <h3>ｽﾞｲ₍₍ (ง ˘ω˘ )ว ⁾⁾ ⁽⁽ 〪ɾ( ˘ω˘ 〫ɩ ) ₎₎ｽﾞｲ</h3>

        <form method="post">
            <input disabled class="button" type="submit" value="加入原趴" name="action">
        </form>
    </div>
</body>

</html>
