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

Go is all over the place, 500-1200mb, GC?

Setting linger.ms to 0 for Rust increased the mem usage of Rust by like 200mb, network overhead?

Go CPU is around 88-100, Rust is around 80-85.

#### Metrics

| Command     | Metric             | Min       | Max       | Mean      | Median    |
| ----------- | ------------------ | --------- | --------- | --------- | --------- |
| rust-single | userTime           | 151.19    | 167.87    | 157.01    | 151.96    |
| go-single   | userTime           | 231.73    | 234.53    | 233.13    | 233.13    |
| rust-single | systemTime         | 103.70    | 116.47    | 109.12    | 107.20    |
| go-single   | systemTime         | 89.95     | 91.28     | 90.62     | 90.62     |
| rust-single | percentCPU         | 84.00     | 85.00     | 84.67     | 85.00     |
| go-single   | percentCPU         | 96.00     | 101.00    | 98.50     | 98.50     |
| rust-single | elapsedTime        | 298.15    | 338.42    | 313.07    | 302.65    |
| go-single   | elapsedTime        | 318.92    | 336.28    | 327.60    | 327.60    |
| rust-single | maxResidentSetSize | 483284.00 | 629160.00 | 540738.67 | 509772.00 |
| go-single   | maxResidentSetSize | 384384.00 | 505656.00 | 445020.00 | 445020.00 |
