#!/usr/local/bin/python

import os
import random
import freshman


FLAG = os.getenv("GZCTF_FLAG", "fake{xxxxxxxx}")

seed = freshman.exit_vim() 
random.seed(seed)

while True:
    number = random.getrandbits(32)
    print("都说了我的种子万无一失，你没办法预测我的随机数")
    ctfer_input = input(f">> ")

    if int(ctfer_input) == number:
        print(FLAG)
        break
    else:
        print(f"猜错了喔，我输入的可是：")
        print(number)

