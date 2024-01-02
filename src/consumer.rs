use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

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

    while let Some(message) = consumer.consumer.stream().next().await {
        match message {
            Ok(msg) => {
                let tailored_msg = match msg.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        format!("Prepared payload: {}, len: {}", payload, payload.len())
                    }
                    Some(Err(_)) => "Message payload is not a string".to_owned(),
                    None => "No payload".to_owned(),
                };

                tokio::spawn(async move {
                    println!("process the msg: {}", &tailored_msg[..42]);
                    tokio::time::sleep(tokio::time::Duration::from_millis(10_000)).await;
                    println!("Done!");
                });
            }
            Err(e) => eprintln!("Error receiving message: {:?}", e),
        }
    }
}
