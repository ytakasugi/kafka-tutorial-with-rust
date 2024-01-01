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

    producer
        .producer
        .send(
            FutureRecord::<(), _>::to(producer.topic).payload("Hello Rust and Kafka"),
            Timeout::Never,
        )
        .await
        .expect("Failed to produce");

    println!("Message sent");
}
