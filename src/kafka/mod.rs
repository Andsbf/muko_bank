pub mod fetch;
pub mod consumer;
pub mod producer;

pub const DEFAULT_BROKER: & str = "127.0.0.1:9093";

pub const DEFAULT_TOPIC: & str = "quickstart-events";

pub const DEFAULT_GROUP: & str = "test-consumer-group";
pub const PROJECTIONS_GROUP: & str = "projection-group";
