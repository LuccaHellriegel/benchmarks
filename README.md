# Benchmarks

## Kafka

Scenario: read 100k msgs and produce them to another topic. Currently only single threaded.

Result: Go (20s) is dramatically faster than Rust (100s). Still trying to figure out why. Very disappointing.
