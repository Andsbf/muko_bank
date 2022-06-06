use crate::domain::Account;
use crate::kafka;
use crate::domain::{Event, Message};

pub fn project(account_id: &str) -> Option<Account> {
    let mut account_stored: Option<Account> = None;

    let message_handler = |message| {
        if let Message::Event(event) = message {
            match event {
                Event::Withdrawn(w) => {
                    if let Some(ref mut account) = account_stored {
                        if w.account_id == account_id {
                            account.balance -= w.amount;
                            account.sequence = w.sequence;
                        }
                    };
                },
                Event::Opened(o) => {
                    if account_stored.is_none() && o.account_id == account_id {
                        account_stored = Some(
                            Account {
                                id: o.account_id,
                                balance: o.balance,
                                name: o.name,
                                created_at: o.time,
                                sequence: o.sequence,
                            }
                        )
                    }
                }
            }
        }
    };

    kafka::fetch::fecth(message_handler);

    account_stored
}
