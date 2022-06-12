use chrono::Utc;
use muko_bank::kafka::{producer, DEFAULT_BROKER, DEFAULT_TOPIC};
use muko_bank::domain::{Command, Withdrawn, Deposited,Opened, Message, Account, WithdrawInsufficientFunds};
use muko_bank::projector::project;
use uuid::Uuid;

pub fn handler(message: Message) {
    if let Message::Command(command) = message {
        println!("Processing command: {:?}", command);

        match command {
            Command::Deposit(deposit) => {
                let account: Account = match project(&deposit.account_id) {
                    Some(account) => account,
                    None => {
                        println!("Account Doesn't exist");
                        return
                    }
                };

                if account.sequence != deposit.sequence {
                    println!("Deposit cancelled, concurrency issue");
                    return
                }

                let data = Deposited {
                    id: Uuid::new_v4().to_string(),
                    deposit_id: deposit.id,
                    account_id: deposit.account_id.to_owned(),
                    amount: deposit.amount,
                    time: Utc::now().to_string(),
                    sequence: account.sequence + 1
                }.as_json();

                if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
                    println!("Failed producing messages: {}", e);
                } else {
                    println!("latest projection {:?}", project(&deposit.account_id).unwrap());
                }

            },
            Command::Withdraw(withdraw) => {
                let account: Account = match project(&withdraw.account_id) {
                    Some(account) => account,
                    None => {
                        println!("Account Doesn't exist");
                        return
                    }
                };

                if account.sequence != withdraw.sequence {
                    println!("Withdraw cancelled, concurrency issue");
                    return
                }

                if account.balance.checked_sub(withdraw.amount).is_none() {
                    let data = WithdrawInsufficientFunds {
                        id: Uuid::new_v4().to_string(),
                         withdraw_id: withdraw.id,
                         time: Utc::now().to_string(),
                         sequence: withdraw.sequence,
                    }.as_json();

                    println!("Withdraw cancelled, insufficient credit");

                    if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
                        println!("Failed producing messages: {}", e);
                    } else {
                        println!("latest projection {:?}", project(&withdraw.account_id).unwrap());
                    }

                    return
                }

                if account.sequence != withdraw.sequence {
                    println!("Withdraw cancelled, concurrency issue");
                    return
                }

                let data = Withdrawn {
                    id: Uuid::new_v4().to_string(),
                    withdraw_id: withdraw.id,
                    account_id: withdraw.account_id.to_owned(),
                    amount: withdraw.amount,
                    time: Utc::now().to_string(),
                    sequence: account.sequence + 1
                }.as_json();

                if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
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
                    id: Uuid::new_v4().to_string(),
                    open_id: o.id,
                    account_id: o.account_id.to_owned(),
                    name: o.name,
                    balance: o.balance,
                    time: now,
                    sequence: 0

                }.as_json();

                if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
                    println!("Failed producing messages: {}", e);
                } else {
                    println!("latest projection {:?}", project(&o.account_id));
                }
            }
        }
    }
}
