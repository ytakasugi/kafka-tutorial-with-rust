use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:19092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    producer
        .send(
            FutureRecord::<(), _>::to("RUST-KAFKA-TOPIC-1").payload("Hello Rust and Kafka")
            , Timeout::Never
        )
        .await
        .expect("Failed to produce");

    println!("Message sent");
}