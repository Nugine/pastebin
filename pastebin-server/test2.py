import requests as rq
import json

payload = {
    "title":"asd",
    "lang":"qwe",
    "content":"zxc",
    "expiration_seconds": 600
}

json.dump(payload, open("post.json","w"))

