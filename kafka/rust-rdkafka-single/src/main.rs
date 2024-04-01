use protobuf::Message as BufMsg;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::error::RDKafkaErrorCode;
use rdkafka::message::{BorrowedHeaders, Headers};
use rdkafka::message::{Message, OwnedHeaders};
use rdkafka::producer::{
    BaseRecord, DefaultProducerContext, NoCustomPartitioner, Producer, ThreadedProducer,
};
use rdkafka::util::Timeout;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

mod proto_message;

fn main() {
    let (tx, rx) = mpsc::channel::<(Vec<u8>, Vec<u8>, Option<OwnedHeaders>)>();

    let producer_thread = thread::spawn(move || {
        let producer: ThreadedProducer<DefaultProducerContext, NoCustomPartitioner> =
            ClientConfig::new()
                .set("bootstrap.servers", "localhost:9092")
                .set("compression.codec", "snappy") //franz-go has snappy compression by default, which is why it was much faster, fml
                .create()
                .expect("Producer creation failed");

        for (key, payload, headers) in rx {
            let m = create_message_from_kafka_record(payload, &headers.unwrap());
            let payload_bytes = m.write_to_bytes().expect("Failed to serialize message");
            let mut b = BaseRecord::to("topicB").payload(&payload_bytes).key(&key);
            loop {
                match producer.send(b) {
                    Err((KafkaError::MessageProduction(RDKafkaErrorCode::QueueFull), bb)) => {
                        thread::sleep(Duration::from_millis(100));
                        b = bb;
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
        // .set("enable.auto.commit", "false")
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
                let headers = m.headers().map(|h| h.detach());
                tx.send((key, payload, headers)).unwrap();
            }
        };
        count += 1;

        if count % 100000 == 0 {
            println!("Processed {}, Elapsed {:?}", count, c.elapsed());
            c = Instant::now();
        }

        if count == 1_000_000 {
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

fn create_message_from_kafka_record(
    payload: Vec<u8>,
    headers: &OwnedHeaders,
) -> proto_message::ProtoMessage {
    let mut msg = proto_message::ProtoMessage::new();
    msg.payload = payload;

    for i in 0..headers.count() {
        let header = headers.get(i);
        let key = header.key;
        let value =
            String::from_utf8(header.value.unwrap_or_default().to_vec()).unwrap_or_default();
        match key {
            "message_type" => msg.message_type = value,
            "some_id" => msg.some_id = value,
            _ => {
                msg.headers.insert(key.to_string(), value);
            }
        }
    }

    if msg.message_type == "" || msg.some_id == "" {
        panic!("broken message")
    }

    msg
}
