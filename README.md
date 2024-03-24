# Benchmarks

## Kafka

### Scenario

Read 100k msgs and produce them to another topic. Commit manually.
Currently only single threaded.

### Result

Go (20s) is dramatically faster than Rust (100s). Still trying to figure out why.

For Rust we see a dramatic slowdown after the first 10k records (120ms to 500ms-1s) and it never gets faster again.
