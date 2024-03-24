use futures::stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<(Vec<u8>, Vec<u8>)>(100);

    // Spawn a new async task to send messages to Kafka
    let producer_task = tokio::spawn(async move {
        let mut count = 0;
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .create()
            .expect("Producer creation failed");

        while let Some((payload, key)) = rx.recv().await {
            producer
                .send(
                    FutureRecord::to("topicB").payload(&payload).key(&key),
                    Duration::from_secs(0),
                )
                .await
                .expect("Failed to produce message");
            count += 1;
            if count % 10000 == 0 {
                println!("Produced {}", count);
            }
            if count == 100_000 {
                break;
            }
        }
        rx.close();
        producer.flush(None).unwrap();
    });

    let consumer_task = tokio::spawn(async move {
        let group_id = format!("rust_{}", chrono::Utc::now().format("%Y%m%d%H%M%S")); //ensures we always start with a new offset
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &group_id)
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "smallest") //mfer this cost me 2 hours because its a default in franz-gp
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");
        consumer
            .subscribe(&["topicA"])
            .expect("Can't subscribe to specified topic");

        let mut count = 0usize;
        println!("start");

        loop {
            match consumer.recv().await {
                Err(e) => panic!("Kafka error: {}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<[u8]>() {
                        None => &[],
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            panic!("Error while deserializing message payload: {:?}", e)
                        }
                    };

                    let key = m.key().unwrap_or(&[]);
                    tx.send((payload.to_vec(), key.to_vec()))
                        .await
                        .expect("Failed to send to channel");

                    count += 1;

                    if count == 100_000 {
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                        break;
                    }

                    if count % 1000 == 0 {
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                    }

                    if count % 10000 == 0 {
                        println!("Processed {}", count);
                    }
                }
            };
        }
        // Shutdown the consumer and producer
        consumer.unsubscribe();
    });

    consumer_task.await.expect("Consumer task failed");
    producer_task.await.expect("Producer task failed");

    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);
}
