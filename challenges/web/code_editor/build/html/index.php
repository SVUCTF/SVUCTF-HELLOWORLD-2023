<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <title>Code Editor</title>
    <link href="https://google-fonts.mirrors.sjtug.sjtu.edu.cn/css2?family=Source+Code+Pro&display=swap"
        rel="stylesheet">
    <link href="https://cdn.bootcdn.net/ajax/libs/prism-themes/1.9.0/prism-one-light.min.css" rel="stylesheet">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            min-height: 100vh;
            background: #F6F8F8;
        }

        main {
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
        }

        header {
            text-align: center;
            margin-top: 50px;
            margin-bottom: 50px;
        }

        .editor {
            background: #fff;
            border-radius: 6px;
            box-shadow: 0 2px 2px 0 rgba(0, 0, 0, 0.14), 0 1px 5px 0 rgba(0, 0, 0, 0.12), 0 3px 1px -2px rgba(0, 0, 0, 0.2);
            font-family: "Source Code Pro", monospace;
            font-size: 14px;
            font-weight: 400;
            min-height: 500px;
            letter-spacing: normal;
            line-height: 20px;
            padding: 20px;
            tab-size: 4;
        }

        #saveButton {
            background-color: rgb(0, 127, 110);
            color: #fff;
            border: none;
            border-radius: 4px;
            padding: 10px 20px;
            font-size: 16px;
            cursor: pointer;
            transition: background-color 0.3s ease;
            display: block;
            margin: 0 auto;
            margin-top: 15px;
        }

        #saveButton:hover {
            background-color: rgb(0, 110, 98)
        }

        #message {
            display: none;
            background-color: #fff;
            border: 1px solid #ddd;
            padding: 10px;
            border-radius: 4px;
            margin-top: 10px;
            font-size: 14px;
            color: #333;
        }
    </style>
</head>

<body>
    <main>
        <header>
            <h1>更适合破晓宝宝的代码编辑器</h1>
        </header>
        <div class="editor" data-manual data-gramm="false"></div>
        <button id="saveButton">保存代码</button>
        <p id="message"></p>
    </main>

    <script type="module">
        import {
            CodeJar
        } from './js/codejar.js'
        const editor = document.querySelector('.editor')

        const highlight = editor => {
            editor.innerHTML = Prism.highlight(editor.textContent, Prism.languages.c, 'c')
        }

        const jar = CodeJar(editor, highlight, {
            tab: '  ',
        })
        const initialCode = `#include <stdio.h>

int main() {
    printf("Welcome to SVUCTF-HELLOWORLD-2023!\\n");
    return 0;
}`;
        jar.updateCode(initialCode);

    </script>

    <script>
        document.addEventListener('DOMContentLoaded', function () {
            const saveButton = document.getElementById('saveButton');
            const editor = document.querySelector('.editor');
            const messageElement = document.getElementById('message');

            saveButton.addEventListener('click', function () {
                const codeToSave = editor.textContent;
                const filename = `code_${Date.now()}.c`;

                fetch('save_code.php', {
                    method: 'POST',
                    body: JSON.stringify({
                        filename: filename,
                        code: codeToSave
                    }),
                    headers: {
                        'Content-Type': 'application/json',
                    },
                })
                    .then(response => response.json())
                    .then(data => {
                        messageElement.style.display = 'block';
                        if (data.success) {
                            messageElement.innerHTML = `因为 OJ 平台还没对接好，先帮你把代码存到 saved/${filename} 啦！`;
                        } else {
                            messageElement.innerHTML = '欸？代码保存失败？';
                        }
                    })
                    .catch(error => {
                        console.error('保存代码时出错:', error);
                    });
            });
        });
    </script>
    <script src="https://cdn.bootcdn.net/ajax/libs/prism/9000.0.1/prism.min.js"></script>
    <script src="https://cdn.bootcdn.net/ajax/libs/prism/9000.0.1/components/prism-c.min.js"></script>
</body>

</html>
