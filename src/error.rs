#[derive(Debug)]
pub enum Error {
    KafkaCreateProducer(rdkafka::error::KafkaError),
    KafkaCreateConsumer(rdkafka::error::KafkaError),
    KafkaSubscribeTopic(rdkafka::error::KafkaError),
    KafkaSendMessage(rdkafka::error::KafkaError),
    KafkaReceiveMessage(rdkafka::error::KafkaError),
    KafkaFetchMetadata(rdkafka::error::KafkaError),
    ReadLine(std::io::Error),
    Other,
}
