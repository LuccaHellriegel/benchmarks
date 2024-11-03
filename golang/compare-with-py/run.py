import subprocess
import time

subprocess.run("cd ./add-go/ && go build -o add && cd ..", shell=True, check=True)


def run(limit):
    print("")
    print(limit)

    def run_command(command, n=25):
        total_real_time = 0
        total_resident_set_size = 0
        total_custom_execution_time = 0

        for _ in range(n):
            start = time.perf_counter()
            process = subprocess.run(
                "UPPER_LIMIT=" + str(limit) + " " + command,
                shell=True,
                text=True,
                stderr=subprocess.PIPE,
                stdout=subprocess.PIPE,
            )
            end = time.perf_counter()

            real_time = end - start
            total_real_time += real_time

            for line in process.stderr.splitlines():
                if "Maximum resident set size" in line:
                    resident_set_size = int(line.split()[-1])
                    total_resident_set_size += resident_set_size

            for line in process.stdout.splitlines():
                if "Execution time:" in line:
                    time_part = line.split(": ")[-1].split("µs")[0]
                    time_value = float(time_part.split()[0])
                    total_custom_execution_time += time_value

        avg_resident_set_size = total_resident_set_size / n
        avg_real_time = total_real_time * 1_000_000 / n
        avg_custom_execution_time = total_custom_execution_time / n

        return {
            "Mem": avg_resident_set_size,
            "Full Time": avg_real_time,
            "Execution Time": avg_custom_execution_time,
        }

    go_results = run_command("/usr/bin/time -v ./add")
    print("Go Results:")
    for key, value in go_results.items():
        print(f"{key}: {value}")

    python_results = run_command("/usr/bin/time -v python3 add.py")
    print("\nPython Naive:")
    for key in go_results:
        print(f"{key}: {(python_results[key] / go_results[key]):.1f}x")

    python_results = run_command("/usr/bin/time -v python3 add_idio.py")
    print("\nPython Idiomatic:")
    for key in go_results:
        print(f"{key}: {(python_results[key] / go_results[key]):.1f}x")

    python_results = run_command("/usr/bin/time -v python3 add_np.py")
    print("\nPython Numpy:")
    for key in go_results:
        print(f"{key}: {(python_results[key] / go_results[key]):.1f}x")


run(100)
run(1000)
run(10000)
run(100000)
