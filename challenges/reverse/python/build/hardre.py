import base64

def encode(message):
    s = ''
    for i in message:
        x = ord(i) - 5
        s += chr(x)

    return base64.b64encode(s.encode())

correct = "YWdcYnZUanBaXG1gWnJkaXg="
flag = ''
print("Please Input flag:")
flag = input()

if encode(flag) == correct:
    print("Win!")
else:
    print("Failed!")
