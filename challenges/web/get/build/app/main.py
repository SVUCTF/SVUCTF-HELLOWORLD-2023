import os
from flask import Flask, request


app = Flask(__name__)

FLAG = os.getenv("GZCTF_FLAG", "flag{test_flag}")


@app.route("/")
def index():
    if want := request.args.get("want"):
        return FLAG
    else:
        return "请用 GET 方式提交一个名为 want，值为 flag 的变量"
