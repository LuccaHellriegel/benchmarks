use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::error::RDKafkaErrorCode;
use rdkafka::message::Message;
use rdkafka::producer::{
    BaseRecord, DefaultProducerContext, NoCustomPartitioner, Producer, ThreadedProducer,
};
use rdkafka::util::Timeout;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let (tx, rx) = mpsc::channel();

    let producer_thread = thread::spawn(move || {
        let producer: ThreadedProducer<DefaultProducerContext, NoCustomPartitioner> =
            ClientConfig::new()
                .set("bootstrap.servers", "localhost:9092")
                .set("compression.codec", "snappy") //franz-go has snappy compression by default, which is why it was much faster, fml
                .create()
                .expect("Producer creation failed");

        for (key, payload) in rx {
            loop {
                match producer.send(BaseRecord::to("topicB").payload(&payload).key(&key)) {
                    Err((KafkaError::MessageProduction(RDKafkaErrorCode::QueueFull), _)) => {
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => {
                        panic!("Error {:?}", e);
                    }
                    Ok(_) => break,
                }
            }
        }
        producer.flush(Timeout::Never).unwrap();
    });

    let group_id = format!("rust_{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
    let consumer: BaseConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "smallest") //took multiple hours to find this lol, franz-go has this as default
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&["topicA"])
        .expect("Can't subscribe to specified topic");

    let start = Instant::now();
    println!("start");

    let mut count = 0usize;
    let mut c = Instant::now();
    loop {
        match consumer.poll(Duration::from_secs(1)) {
            None => continue,
            Some(m) => {
                let m = m.unwrap();
                let payload = m.payload().unwrap().to_vec();
                let key = m.key().unwrap_or(&[]).to_vec();
                tx.send((key, payload)).unwrap();
            }
        };
        count += 1;

        if count % 10000 == 0 {
            println!("Processed {}, Elapsed {:?}", count, c.elapsed());
            c = Instant::now();
        }

        if count == 100_000 {
            break;
        }
    }

    // Shutdown the consumer and producer
    consumer.unsubscribe();
    drop(tx); // Close the channel to stop the producer thread
    producer_thread.join().unwrap();
    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);
}
