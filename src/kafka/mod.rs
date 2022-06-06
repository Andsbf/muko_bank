pub mod fetch;
pub mod consumer;
pub mod producer;

pub const DEFAULT_BROKER: & str = "localhost:9092";
pub const DEFAULT_TOPIC: & str = "quickstart-events";
pub const DEFAULT_GROUP: & str = "test-consumer-group";
