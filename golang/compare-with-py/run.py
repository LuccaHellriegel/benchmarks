import subprocess
import time


def run_command(command, n=25):
    total_real_time = 0
    total_resident_set_size = 0
    total_custom_execution_time = (
        0  # To accumulate custom execution times printed by the program
    )

    for _ in range(n):
        start = time.perf_counter()
        process = subprocess.run(
            command,
            shell=True,
            text=True,
            stderr=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        end = time.perf_counter()

        real_time = end - start
        total_real_time += real_time

        # Extract memory usage from stderr
        for line in process.stderr.splitlines():
            if "Maximum resident set size" in line:
                resident_set_size = int(
                    line.split()[-1]
                )  # Get the last element assuming it's the number
                total_resident_set_size += resident_set_size

        # Extract custom execution time from stdout
        for line in process.stdout.splitlines():
            if "Execution time:" in line:
                # Extract time and convert it to seconds
                time_part = line.split(": ")[-1].split("µs")[0]
                time_value = float(time_part.split()[0])  # Get the numeric part
                total_custom_execution_time += time_value

    # Compute averages
    avg_resident_set_size = total_resident_set_size / n
    avg_real_time = total_real_time * 1_000_000 / n
    avg_custom_execution_time = total_custom_execution_time / n

    return {
        "Average Maximum Resident Set Size (kbytes)": avg_resident_set_size,
        "Average Real Time (µs)": avg_real_time,
        "Average Execution Time (µs)": avg_custom_execution_time,
    }


# Build the Go binary
subprocess.run("cd ./add-go/ && go build -o add && cd ..", shell=True, check=True)

# Execute and average results for the Go program
go_results = run_command("/usr/bin/time -v ./add")
print("Go Program Results:")
for key, value in go_results.items():
    print(f"{key}: {value}")

# Execute and average results for the Python script
python_results = run_command("/usr/bin/time -v python3 add.py")
# print("\nPython Script Results:")
# for key, value in python_results.items():
#     print(f"{key}: {value}")

print("\nDiff(Python vs Go):")
for key in go_results:
    print(f"{key}: {(python_results[key] / go_results[key]):.1f}x")


# Execute and average results for the Python script
python_results = run_command("/usr/bin/time -v python3 add_idiomatic.py")
# print("\nPython Idiomatic Script Results:")
# for key, value in python_results.items():
#     print(f"{key}: {value}")

print("\nDiff(Python Idiomatic vs Go):")
for key in go_results:
    print(f"{key}: {(python_results[key] / go_results[key]):.1f}x")


# Execute and average results for the Python script
python_results = run_command("/usr/bin/time -v python3 add_np.py")
# print("\nPython Numpy Script Results:")
# for key, value in python_results.items():
#     print(f"{key}: {value}")

print("\nDiff(Python Numpy vs Go):")
for key in go_results:
    print(f"{key}: {(python_results[key] / go_results[key]):.1f}x")
