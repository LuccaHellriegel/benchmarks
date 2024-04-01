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

#### Conclusion

- Rust takes 10% less CPU
- Rust takes roughly the same time
- Rust seems to take a little more memory, however, the memory is much stabler - e.g. Min (Go: 400mb, Rust: 560mb), Max (Go: 1260mb, Rust: 760mb)

#### Notes

Memory usage depends on how many messages need to be buffered - either currently consuming or producing.

Go is all over the place, 500-1200mb, GC?

Setting linger.ms to 0 for Rust increased the mem usage of Rust by like 200mb, network overhead?

Go CPU is around 88-100, Rust is around 80-85.

#### Metrics

| Command     | Metric             | Min       | Max        | Mean      | Median    |
| ----------- | ------------------ | --------- | ---------- | --------- | --------- |
| go-single   | userTime           | 191.44    | 213.77     | 196.73    | 193.16    |
| rust-single | userTime           | 132.84    | 147.51     | 136.37    | 135.56    |
| go-single   | systemTime         | 74.77     | 85.10      | 77.42     | 76.10     |
| rust-single | systemTime         | 86.20     | 97.16      | 88.83     | 88.16     |
| go-single   | percentCPU         | 99.00     | 105.00     | 103.92    | 105.00    |
| rust-single | percentCPU         | 84.00     | 89.00      | 87.46     | 88.00     |
| go-single   | elapsedTime        | 254.87    | 298.98     | 262.52    | 256.70    |
| rust-single | elapsedTime        | 249.74    | 290.76     | 255.76    | 252.77    |
| go-single   | maxResidentSetSize | 402192.00 | 1260940.00 | 768365.85 | 725124.00 |
| rust-single | maxResidentSetSize | 569596.00 | 763076.00  | 720986.77 | 729944.00 |
