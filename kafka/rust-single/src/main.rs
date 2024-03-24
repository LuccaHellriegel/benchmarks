use rdkafka::config::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::BaseProducer;
use rdkafka::producer::BaseRecord;
use rdkafka::producer::Producer;
use std::time::Instant;

fn main() {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    let group_id = format!("rust_{}", chrono::Utc::now().format("%Y%m%d%H%M%S")); //ensures we always start with a new offset
    let consumer: BaseConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "smallest") //mfer this cost me 2 hours because its a default in franz-gp
        // .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&["topicA"])
        .expect("Can't subscribe to specified topic");

    let mut count = 0usize;
    let start = Instant::now();
    println!("start");

    loop {
        match consumer.poll(None) {
            None => continue,
            Some(m) => {
                let m = m.unwrap();
                producer
                    .send(
                        BaseRecord::to("topicB")
                            .payload(m.payload().unwrap())
                            .key(m.key().unwrap_or(&[])),
                    )
                    .expect("Failed to produce message");

                count += 1;

                if count % 10 == 0 {
                    producer.poll(None);
                }

                if count % 10000 == 0 {
                    println!("Processed {}", count);
                }

                if count == 100_000 {
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                    break;
                }

                if count % 1000 == 0 {
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            }
        };
    }
    // Shutdown the consumer and producer
    consumer.unsubscribe();
    producer.flush(None).unwrap();

    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);
}
