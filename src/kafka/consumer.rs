use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

use crate::domain::Message;
use crate::kafka as kafkaBank;

pub fn consume_messages(message_handler: fn(Message)) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(vec![kafkaBank::DEFAULT_BROKER.to_string()])
        .with_topic(kafkaBank::DEFAULT_TOPIC.to_string())
        .with_group(kafkaBank::DEFAULT_GROUP.to_string())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    loop {
        let mss = con.poll()?;
        // if mss.is_empty() {
        //     println!("No messages available right now.");
        //     return Ok(());
        // }

        for ms in mss.iter() {
            for m in ms.messages() {
                // println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, std::str::from_utf8(m.value).unwrap());
                let message = std::str::from_utf8(m.value).unwrap();

                let operation: Message  = serde_json::from_str(message).unwrap();

                message_handler(operation)
            }

            let _ = con.consume_messageset(ms);
            con.commit_consumed()?;
        }
    }
}
