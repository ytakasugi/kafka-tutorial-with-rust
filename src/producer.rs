use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;

#[derive(Clone)]
struct KafkaProducer<'a> {
    producer: FutureProducer,
    topic: &'a str,
}

impl<'a> KafkaProducer<'a> {
    fn new(topic: &'a str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:29092")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Self {
            producer: producer,
            topic: topic,
        }
    }
}

#[tokio::main]
async fn main() {
    let producer: KafkaProducer = KafkaProducer::new("RUST-KAFKA-TOPIC-1");
    let mut tasks = Vec::new();

    for _ in 0..10 {
        let message = "kafka tutorial with Rust".to_string();
        let producer_clone = producer.producer.clone();

        let task = tokio::spawn(async move {
            let record = FutureRecord::to(producer.topic)
                .key("kafka-tutorial-with-rust-key")
                .payload(&message);

            match producer_clone.send(record, Timeout::Never).await {
                Ok(delivery) => println!("Sent: {:?}", delivery),
                Err((e, _)) => eprintln!("Error: {:?}", e),
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.expect("Task failed to complete");
    }

    println!("All messages sent");
}
