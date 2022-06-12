use kafka::client::{FetchPartition, KafkaClient};

use super::super::domain::Message;
use super::DEFAULT_BROKER;
use super::DEFAULT_TOPIC;

pub fn fecth<F>(mut message_handler: F)
where
    F: FnMut(Message),
{
    let topic = DEFAULT_TOPIC.to_owned();
    let partition = 0;
    let offset = 0;

    // println!(
    //     "About to fetch messages at {} from: {} (partition {}, offset {}) ",
    //     broker, topic, partition, offset
    // );

    let mut client = KafkaClient::new(vec![DEFAULT_BROKER.to_owned()]);
    if let Err(e) = client.load_metadata_all() {
        println!("Failed to load metadata from {}: {}", DEFAULT_BROKER, e);
    }

    // ~ make sure to print out a warning message when the target
    // topic does not yet exist
    if !client.topics().contains(&topic) {
        println!("No such topic at {}: {}", DEFAULT_BROKER, topic);
    }

    match client.fetch_messages(&[FetchPartition::new(&topic, partition, offset)]) {
        Err(e) => {
            println!("Failed to fetch messages: {}", e);
        }
        Ok(resps) => {
            for resp in resps {
                for t in resp.topics() {
                    for p in t.partitions() {
                        match p.data() {
                            Err(ref e) => {
                                println!("partition error: {}:{}: {}", t.topic(), p.partition(), e)
                            }
                            Ok(data) => {
                                // println!(
                                //     "topic: {} / partition: {} / latest available message \
                                //           offset: {}",
                                //     t.topic(),
                                //     p.partition(),
                                //     data.highwatermark_offset()
                                // );
                                for msg in data.messages() {
                                    let msg_value_as_str = std::str::from_utf8(msg.value).unwrap();

                                    // println!(
                                    //     "topic: {} / partition: {} / message.offset: {} / \
                                    //           message.len: {}",
                                    //     t.topic(),
                                    //     p.partition(),
                                    //     msg.offset,
                                    //     msg_value_as_str
                                    // );

                                    let message: Message  = serde_json::from_str(msg_value_as_str).unwrap();

                                    message_handler(message);
                                }

                            }
                        }
                    }
                }
            }
        }
    }
}
