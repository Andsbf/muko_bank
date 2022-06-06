#!/bin/sh


/Users/ando/kafka/kafka_2.13-3.2.0/bin/kafka-topics.sh  --delete --topic quickstart-events  --bootstrap-server localhost:9092
/Users/ando/kafka/kafka_2.13-3.2.0/bin/kafka-topics.sh --create --topic quickstart-events --bootstrap-server localhost:9092
