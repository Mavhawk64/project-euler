import os

with open(
    os.path.join(os.path.dirname(__file__), "names.txt"), "r", encoding="utf-8"
) as f:
    output = [i.split(",") for i in f.read().split("\n")]

print(output)
