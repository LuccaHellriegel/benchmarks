# Benchmarks

## Kafka

### Scenario

Read 100k msgs, create a protobuf struct and produce it to another topic. Commit manually.
Currently only two threaded.

### Result

#### Go

Command being timed: "./dist/go-single"
User time (seconds): 179.46
System time (seconds): 70.54
Percent of CPU this job got: 108%
Elapsed (wall clock) time (h:mm:ss or m:ss): 3:50.18
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 1124544 / 1124,544 mbytes
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 0
Minor (reclaiming a frame) page faults: 14755278
Voluntary context switches: 2427389
Involuntary context switches: 19959
Swaps: 0
File system inputs: 0
File system outputs: 0
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0

#### Rust

Command being timed: "./dist/release/rust-single"
User time (seconds): 117.12
System time (seconds): 80.54
Percent of CPU this job got: 88%
Elapsed (wall clock) time (h:mm:ss or m:ss): 3:44.25
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 575084 / 575,084 mbytes
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 0
Minor (reclaiming a frame) page faults: 26678057
Voluntary context switches: 558095
Involuntary context switches: 8628
Swaps: 0
File system inputs: 0
File system outputs: 0
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0
