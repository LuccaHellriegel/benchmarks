import time
import numpy as np

print("Python")

upper_limit = int(os.getenv("UPPER_LIMIT"))
start_time = time.time()

# Creating an array using NumPy for faster computation
data = np.arange(upper_limit)

# Directly using NumPy operations for summing doubles
res = np.sum(data)

end_time = time.time()

print("Result:", 0)
print("Execution time:", (end_time - start_time) * 1_000_000, "µs")
