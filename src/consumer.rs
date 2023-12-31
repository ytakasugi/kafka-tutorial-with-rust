use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};

use futures::TryStreamExt;

#[tokio::main]
async fn main() {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:19092")
        .set("enable.partition.eof", "false")
        .set("group.id", "rust-kafka-group-1")
        .create::<StreamConsumer>()
        .expect("Failed to create client");

    consumer.subscribe(&["RUST-KAFKA-TOPIC-1"]).unwrap();

    let stream_processor = consumer.stream().try_for_each(|msg| {
        async move {
            println!("Received message from topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                     msg.topic(), msg.partition(), msg.offset(), msg.timestamp());

            tokio::time::sleep(tokio::time::Duration::from_millis(3_000)).await;
            let r = match msg.payload_view::<str>() {
                Some(Ok(payload)) => format!("Payload len for {} is {}", payload, payload.len()),
                Some(Err(_)) => "Message payload is not a string".to_owned(),
                None => "No payload".to_owned(),
            };
            println!("{}", r);
            Ok(())
        }
    });

    stream_processor.await.expect("stream processing failed");
}