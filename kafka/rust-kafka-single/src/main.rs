use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::producer::{Producer, Record};
use std::time::Duration;
use std::time::Instant;

fn main() {
    // Configure the consumer
    let group_id = format!("rust_{}", chrono::Utc::now().format("%Y%m%d%H%M%S")); //ensures we always start with a new offset

    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic_partitions("topicA".to_owned(), &[0])
        .with_group(group_id)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .with_fallback_offset(FetchOffset::Earliest)
        .create()
        .expect("Consumer creation failed");

    // Configure the producer
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .create()
        .expect("Producer creation failed");

    let mut count = 0usize;
    let start = Instant::now();
    println!("start");

    let mut c = Instant::now();
    loop {
        // Poll messages from topicA
        let message_sets = consumer.poll().expect("Failed to poll messages");

        let msgs: Vec<Record<(), &[u8]>> = message_sets
            .iter()
            .flat_map(|set| set.messages())
            .map(|msg| Record::from_value("topicB", msg.value))
            .collect();
        producer
            .send_all(msgs.as_slice())
            .expect("Failed to send message");
        let l = msgs.len();
        count += l;

        if count / 1000 > (count - l) / 1000 {
            println!("Processed {}, Elapsed {:?}", count, c.elapsed());
            c = Instant::now();
        }

        // Commit offsets for the messages polled
        consumer
            .commit_consumed()
            .expect("Failed to commit offsets");

        if count == 100_000 {
            break;
        }
    }

    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);
}
