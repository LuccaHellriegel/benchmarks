import time
import os

print("Python")

upper_limit = int(os.getenv("UPPER_LIMIT"))
start_time = time.time()

# Using list comprehension for faster data generation
data = [i for i in range(upper_limit)]

# Utilizing the sum function with a generator expression for efficient summation
res = sum(d for d in data)

end_time = time.time()

print("Result:", res)
print("Execution time:", (end_time - start_time) * 1_000_000, "µs")
