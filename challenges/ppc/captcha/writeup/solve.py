import requests
from lxml import etree


url = "http://localhost"
session = requests.Session()

resp = session.get(url)
html = etree.HTML(resp.text)
expression = html.xpath("/html/body/p[@id='expression']/text()")[0]
print(expression)

answer = eval(expression)
resp = session.post(f"{url}/verify", data={"answer": answer})
print(resp.json())
