use chrono::Utc;
use muko_bank::kafka::producer;
use muko_bank::domain::{Command, Withdrawn, Opened, Message, Account};
use muko_bank::projector::project;

pub fn handler(message: Message) {
    if let Message::Command(command) = message {
        println!("Processing command: {:?}", command);

        match command {
            Command::Withdraw(withdraw) => {
                let account: Account = match project(&withdraw.account_id) {
                    Some(account) => account,
                    None => {
                        println!("Withdraw cancelled, insufficient credit");
                        return
                    }
                };

                if account.balance.checked_sub(withdraw.amount).is_none() {
                    println!("Withdraw cancelled, insufficient credit");
                    return
                }

                if account.sequence != withdraw.sequence {
                    println!("Withdraw cancelled, concurrency issue");
                    return
                }

                let data = Withdrawn {
                    account_id: withdraw.account_id.to_owned(),
                    amount: withdraw.amount,
                    time: Utc::now().to_string(),
                    sequence: account.sequence + 1
                }.as_json();


                let broker = "localhost:9092".to_owned();
                let topic = "quickstart-events".to_owned();

                if let Err(e) = producer::produce_message(data.as_bytes(), &topic, vec![broker]) {
                    println!("Failed producing messages: {}", e);
                } else {
                    println!("latest projection {:?}", project(&withdraw.account_id).unwrap());
                }

            },
            Command::Open(o) => {
                if let Some(a) = project(&o.account_id) {
                    println!("Open cancelled, Account already exists, {:?}",a);
                    return
                }

                let now = Utc::now().to_string();
                let data = Opened {
                    account_id: o.account_id.to_owned(),
                    name: o.name,
                    balance: o.balance,
                    time: now,
                    sequence: 0

                }.as_json();

                let broker = "localhost:9092".to_owned();
                let topic = "quickstart-events".to_owned();

                if let Err(e) = producer::produce_message(data.as_bytes(), &topic, vec![broker]) {
                    println!("Failed producing messages: {}", e);
                } else {
                    println!("latest projection {:?}", project(&o.account_id));
                }
            }
        }
    }
}
