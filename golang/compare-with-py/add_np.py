import time
import numpy as np

print("Python")

start_time = time.time()

# Creating an array using NumPy for faster computation
data = np.arange(1000)

# Directly using NumPy operations for summing doubles
res = np.sum(data * 2)

end_time = time.time()

print("Result:", 0)
print("Execution time:", (end_time - start_time) * 1_000_000, "µs")
