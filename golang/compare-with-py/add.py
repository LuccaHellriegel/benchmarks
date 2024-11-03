import time

print("Python")

upper_limit = int(os.getenv("UPPER_LIMIT"))
start_time = time.time()

data = []
for i in range(upper_limit):
    data.append(i)

res = 0
for d in data:
    res += d

end_time = time.time()
print("Result:", res)
print("Execution time:", (end_time - start_time) * 1_000_000, "µs")
