import requests as rq
import json
import os

payload = {
    "title":"asd",
    "lang":"qwe",
    "content":"zxc",
    "expiration_seconds": 600
}

url = "http://localhost:6000"

res = rq.post(url+"/records",json=payload)

if res.status_code == 200:
    key = res.json()['key']
    target = f"{url}/records/{key}"
    print(target)
else:
    print(res)
    print(res.headers)
    print(res.json())  
    exit(1)

os.system(f"ab -k -v 1 -n 1000000 -c 128 {target}")
os.system(f"curl {target}")
# os.system(f"wrk -t16 -c128 -d10s --latency {target}")
os.system(f"curl {target}")
