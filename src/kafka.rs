use crate::{chat_message::ChatMessage, error::Error};
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};
use std::time::Duration;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Kafka {
    server: String,
    pub topic: String,
}

impl Kafka {
    pub fn new(server: &str, topic: &str) -> Self {
        Self {
            server: server.into(),
            topic: topic.into(),
        }
    }

    fn create_producer(&self) -> Result<FutureProducer, Error> {
        ClientConfig::new()
            .set("bootstrap.servers", self.server.to_owned())
            .set("message.timeout.ms", "5000")
            .create()
            .map_err(Error::KafkaCreateProducer)
    }

    fn create_consumer(&self) -> Result<StreamConsumer, Error> {
        ClientConfig::new()
            .set("group.id", "consumer_cli")
            .set("bootstrap.servers", self.server.to_owned())
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .map_err(Error::KafkaCreateConsumer)
    }

    /// Sends a message to the Kafka topic.
    pub async fn produce(&self, content: &str) -> Result<(), Error> {
        let producer: &FutureProducer = &self.create_producer()?;

        let key = Uuid::new_v4().to_string();
        let message = ChatMessage::new(content.into()).to_string();

        let _ = producer
            .send(
                FutureRecord::to(&self.topic).payload(&message).key(&key),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(e, _)| Error::KafkaSendMessage(e))?;

        Ok(())
    }

    /// Consumes messages from the Kafka topic and calls the `receive_fn` for each message.
    pub async fn consume<F>(&self, receive_fn: F) -> Result<(), Error>
    where
        F: Fn(&str),
    {
        let consumer: &StreamConsumer = &self.create_consumer()?;
        let topics = &[self.topic.as_str()];

        consumer
            .subscribe(topics)
            .map_err(Error::KafkaSubscribeTopic)?;

        loop {
            match consumer.recv().await {
                Ok(message) => {
                    if let Some(Ok(payload)) = message.payload_view::<str>() {
                        receive_fn(payload);
                    }
                }
                Err(e) => warn!("error received: {:?}", Error::KafkaReceiveMessage(e)),
            }
        }
    }
}
