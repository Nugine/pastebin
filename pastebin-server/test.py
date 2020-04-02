import requests as rq
import json

payload = {
    "title":"asd",
    "lang":"qwe",
    "content":"zxc",
    "expiration_seconds": 600
}

url = "http://localhost:3000"

res = rq.post(url+"/record",json=payload)

if res.status_code == 200:
    key = res.json()['key']
    print(f"{url}/record/{key}")
else:
    print(res)
    print(res.headers)
    print(res.json())   
