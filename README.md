# Benchmarks

## Kafka

### Scenario

Message source: produced as fast as possible in parallel to the actual application.

Application: Read X msgs, create a protobuf struct and produce it to another topic. Commit automatically.
Currently 1 thread for consumer, 1 for producer.
Approximates at least one use case I was thinking about using Rust for.

.fresh.sh cleans and starts up a fresh kafka

measure-xx.sh starts up the message production and the application and prints time / mem taken at the end

### Result

Memory usage depends on how many messages need to be buffered - either currently consuming or producing.
Setting linger.ms to 0 for Rust increased the mem usage of Rust by like 200mb.

#### 1 Mil - Go

Command being timed: "./dist/go-single"
User time (seconds): 234.53
System time (seconds): 89.95
Percent of CPU this job got: 96%
Elapsed (wall clock) time (h:mm:ss or m:ss): 5:36.28
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 505656
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 0
Minor (reclaiming a frame) page faults: 14101075
Voluntary context switches: 2606919
Involuntary context switches: 93624
Swaps: 0
File system inputs: 8
File system outputs: 88
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0

#### 1 Mil - Rust

Command being timed: "./dist/release/rust-single"
User time (seconds): 151.19
System time (seconds): 107.20
Percent of CPU this job got: 85%
Elapsed (wall clock) time (h:mm:ss or m:ss): 5:02.65
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 509772
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 0
Minor (reclaiming a frame) page faults: 27189437
Voluntary context switches: 627750
Involuntary context switches: 34677
Swaps: 0
File system inputs: 0
File system outputs: 88
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0
