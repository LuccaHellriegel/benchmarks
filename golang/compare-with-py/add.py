import time

print("Python")

start_time = time.time()

data = []
for i in range(1000):
    data.append(i)

res = 0
for d in data:
    res += d * 2

end_time = time.time()
print("Result:", res)
print("Execution time:", (end_time - start_time) * 1_000_000, "µs")
