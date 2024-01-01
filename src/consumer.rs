use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

use futures::TryStreamExt;

struct KafkaConsumer<'a> {
    consumer: StreamConsumer,
    topic: &'a str,
}

impl<'a> KafkaConsumer<'a> {
    fn new(topic: &'a str) -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:29092")
            .set("enable.partition.eof", "false")
            .set("group.id", "rust-kafka-group-1")
            .create::<StreamConsumer>()
            .expect("Failed to create client");

        Self {
            consumer: consumer,
            topic: topic,
        }
    }
}

#[tokio::main]
async fn main() {
    let consumer: KafkaConsumer = KafkaConsumer::new("RUST-KAFKA-TOPIC-1");

    consumer.consumer.subscribe(&[consumer.topic]).unwrap();

    let stream_processor = consumer.consumer.stream().try_for_each(|msg| async move {
        println!(
            "Received message from topic: {}, partition: {}, offset: {}, timestamp: {:?}",
            msg.topic(),
            msg.partition(),
            msg.offset(),
            msg.timestamp()
        );

        tokio::time::sleep(tokio::time::Duration::from_millis(3_000)).await;
        let r = match msg.payload_view::<str>() {
            Some(Ok(payload)) => format!("Payload len for {} is {}", payload, payload.len()),
            Some(Err(_)) => "Message payload is not a string".to_owned(),
            None => "No payload".to_owned(),
        };
        println!("{}", r);
        Ok(())
    });

    stream_processor.await.expect("stream processing failed");
}
