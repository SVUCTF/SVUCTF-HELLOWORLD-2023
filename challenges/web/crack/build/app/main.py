import os
from flask import Flask, request, render_template


app = Flask(__name__)

FLAG = os.getenv("GZCTF_FLAG", "flag{test_flag}")


@app.route("/")
def index():
    return render_template("index.html")


@app.route("/login", methods=["POST"])
def login():
    username = request.form.get("username")
    password = request.form.get("password")

    if username == "admin" and password == "password":
        return f"Welcome, {username}! The flag is: {FLAG}"
    else:
        return f"Wrong username or password, only admin can view flag"