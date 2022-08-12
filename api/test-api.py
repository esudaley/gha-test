from fastapi import FastAPI
import redis 
import os

app = FastAPI()
h = "redis" if "INSIDEDOCKER" in os.environ else "localhost"
print(h)
r = redis.Redis(host=h, port=6379)
r.set("test", "Hello World!")

@app.get("/")
def root ():
# create connection to the redis cluster
  test = r.get("test")
  return {"message": test}