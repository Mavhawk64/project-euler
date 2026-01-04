import time

MIN = 2
MAX = 1000

start = time.perf_counter()

ret = set()
for a in range(MIN, MAX + 1):
    tmp = []
    for b in range(MIN, MAX + 1):
        tmp.append(int(a**b))
    ret |= set(tmp)

end = time.perf_counter()

print(f"Result: {len(ret)}")
print(f"Time: {end - start:.4f} seconds")
