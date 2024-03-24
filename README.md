# Benchmarks

## Kafka

### Scenario

Read 100k msgs and produce them to another topic. Commit manually.
Currently only single threaded.

### Result

Go (20s) is dramatically faster than Rust (100s). Still trying to figure out why.

For Rust we see a dramatic slowdown after the first 10k records (120ms to 500ms-1s) and it never gets faster again.

This is for rust-rdkafka. For rust-kafka, due to the synchronous sending the times are even worse. We would need to implement a full thread infra to execute it.

Conclusion: the Kafka DX in Rust is way worse at the moment. A comparable speed in a single-threaded approach (franz-go probably does some goroutines under the hood) is not achievable in a useful timespan.
