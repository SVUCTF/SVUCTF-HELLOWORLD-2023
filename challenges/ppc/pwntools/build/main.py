#!/usr/local/bin/python

import os
import random

FLAG = os.getenv("GZCTF_FLAG", "fake{xxxxxxxxxx}")
CHOICES = ["石头", "剪刀", "布"]

def main():
    for i in range(500):
        zzz_choice = random.choice(CHOICES)
        print(f"@zzz 同学好像要出 {zzz_choice}")

        user_choice = input("选择石头、剪刀或布：")
        if user_choice not in CHOICES:
            print("只可以输入石头、剪刀或者布哦。")

        if (
            (user_choice == "石头" and zzz_choice == "剪刀")
            or (user_choice == "剪刀" and zzz_choice == "布")
            or (user_choice == "布" and zzz_choice == "石头")
        ):
            print(f"@zzz: 卡了...只是卡了...你还要赢我 {500 - i} 次")
        else:
            print("@zzz: 我太强辣！")
            exit(0)

    print(f"欺负我高 Ping 战士，Flag 给你：{FLAG}")


if __name__ == "__main__":
    main()
