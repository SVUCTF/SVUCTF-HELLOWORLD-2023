import base64

correct = "YWdcYnZUanBaXG1gWnJkaXg="

message = base64.b64decode(correct).decode()

flag = ""
for i in message:
    x = ord(i) + 5
    flag += chr(x)

print(flag)
