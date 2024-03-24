use rdkafka::config::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::error::RDKafkaErrorCode;
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;
use rdkafka::producer::BaseProducer;
use rdkafka::producer::BaseRecord;
use rdkafka::producer::Producer;
use rdkafka::util::Timeout;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let producer: BaseProducer = ClientConfig::new()
        // .set("linger.ms", "0")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    let group_id = format!("rust_{}", chrono::Utc::now().format("%Y%m%d%H%M%S")); //ensures we always start with a new offset
    let consumer: BaseConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "smallest")
        // .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&["topicA"])
        .expect("Can't subscribe to specified topic");

    let mut count = 0usize;
    let start = Instant::now();
    println!("start");

    let mut c = Instant::now();
    loop {
        match consumer.poll(Duration::from_secs(1)) {
            None => continue,
            Some(m) => {
                let m = m.unwrap();
                let payload = m.payload().unwrap();
                let key = m.key().unwrap_or(&[]);
                loop {
                    match producer.send(BaseRecord::to("topicB").payload(payload).key(key)) {
                        Err((KafkaError::MessageProduction(RDKafkaErrorCode::QueueFull), _)) => {
                            producer.poll(Duration::from_millis(10));
                            continue;
                        }
                        Err(e) => {
                            panic!("Error {:?}", e);
                        }
                        Ok(_) => break,
                    }
                }
                producer.poll(Duration::from_secs(0));
            }
        };
        count += 1;

        // if count % 10000 == 0 {
        //     producer.poll(Duration::from_millis(100));
        // }

        if count % 1000 == 0 {
            println!("Processed {}, Elapsed {:?}", count, c.elapsed());
            c = Instant::now();
        }

        if count == 100_000 {
            break;
        }
    }
    producer.flush(Timeout::Never).unwrap();
    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);

    // Shutdown the consumer and producer
    consumer.unsubscribe();
}
