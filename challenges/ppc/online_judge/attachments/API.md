# [PPC] 更适合破晓宝宝的 OJ API 文档

程序只有两个 API ，URL 路径都是 `/jobs`。

GET 请求返回全部已提交的任务，返回它们的状态和结果：

```json
[
  {
    "id": 0,
    "created_time": "2023-10-16T18:34:16.287Z",
    "updated_time": "2023-10-16T18:34:16.287Z",
    "submission": {
      "source_code": "#include <stdio.h>\nint main() {\nprintf(\"Hello World!\");\nreturn 0;}",
      "language": "C",
      "problem_id": 0
    },
    "state": "Finished",
    "result": "Accepted",
    "cases": [
      {
        "result": "Compilation Success",
        "time": 0,
        "memory": 0
      },
      {
        "result": "Accepted",
        "time": 0,
        "memory": 0
      }
    ],
    "flag": "flag{"
  }，
  {
    "id": 1,
    "created_time": "2023-10-16T19:41:05.106Z",
    "updated_time": "2023-10-16T19:41:05.106Z",
    "submission": {
      "source_code": "#include <stdio.h>\nint main() {\nprintf(\"Hello World!\");\nreturn }",
      "language": "C",
      "problem_id": 0
    },
    "state": "Finished",
    "result": "Compilation Error",
    "cases": [
      {
        "result": "Compilation Error",
        "time": 0,
        "memory": 0
      }
    ],
    "flag": ""
  }
]
```

---

POST 请求提交 语言类型、题目 ID 、代码内容 ，返回此次评测结果：

REQUEST：

```json
{
    "language": "C",
    "problem_id": 0,
    "source_code": "#include <stdio.h>\nint main() {\nprintf(\"Hello World!\");\nreturn 0;}",
}
```

RESPONSE：

```json
{
  "id": 3,
  "created_time": "2023-10-16T19:39:33.541Z",
  "updated_time": "2023-10-16T19:39:33.541Z",
  "submission": {
    "source_code": "#include <stdio.h>\nint main() {\nprintf(\"Hello World!\");\nreturn 0;}",
    "language": "C",
    "problem_id": 0
  },
  "state": "Finished",
  "result": "Accepted",
  "cases": [
    {
      "result": "Compilation Success",
      "time": 0,
      "memory": 0
    },
    {
      "result": "Accepted",
      "time": 0,
      "memory": 0
    }
  ],
  "flag": "flag{"
}
```

