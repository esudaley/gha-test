import os
print("redis" if "INSIDEDOCKER" in os.environ else "localhost")

