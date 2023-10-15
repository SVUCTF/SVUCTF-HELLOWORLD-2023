import json
import requests


# 你要替换 localhost 和 12345 为你的 IP 和 端口
url = "http://localhost:12345/jobs"

# 从源码文件读取代码
with open("test.c") as f:
    code = f.read()

# 提交代码
data = {
    "language": "C",  # 语言 C / CPP
    "problem_id": 0,  # 问题 ID
    "source_code": code,
}
resp = requests.post(url=url, json=data)

# 获取所有结果（所有提交过的任务）
resp = requests.get(url=url)
resp_json = resp.json()
print(json.dumps(resp_json, indent=2))
