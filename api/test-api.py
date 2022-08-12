from fastapi import FastAPI
import redis 
app = FastAPI()

r = redis.Redis(host='redis', port=6379)
r.set("test", "Hello World!")

@app.get("/")
def root ():
# create connection to the redis cluster
  test = r.get("test")
  return {"message": test}