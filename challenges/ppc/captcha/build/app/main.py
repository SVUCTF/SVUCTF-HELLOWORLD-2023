from flask import Flask, render_template, request, jsonify, session
import random
import time
import os

app = Flask(__name__)
app.secret_key = os.urandom(24)
FLAG = os.getenv("GZCTF_FLAG")


@app.route("/")
def index():
    num1 = random.randint(10000000, 1000000000)
    num2 = random.randint(10000000, 1000000000)
    operator = random.choice(["+", "-"])
    expression = f"{num1} {operator} {num2}"

    if operator == "+":
        correct_answer = num1 + num2
    else:
        correct_answer = num1 - num2

    session["start_time"] = time.time()
    session["correct_answer"] = correct_answer

    return render_template("index.html", expression=expression)


@app.route("/verify", methods=["POST"])
def verify():
    user_answer = request.form["answer"]

    if time.time() - session["start_time"] > 2:
        return jsonify({"result": "Timeout"})
    if int(user_answer) != session["correct_answer"]:
        return jsonify({"result": "incorrect"})
    else:
        return jsonify({"result": "correct", "flag": FLAG})
