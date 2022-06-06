use std::io;
use chrono::prelude::*;
use terminal_menu::{menu, label, button, run, mut_menu};

use muko_bank::kafka::{producer, DEFAULT_TOPIC, DEFAULT_BROKER};
use muko_bank::projector;
use muko_bank::domain::{Withdraw, Account};

fn main() {
    println!("Please input account_id.");

    let mut account_id = String::new();

    io::stdin()
        .read_line(&mut account_id)
        .expect("Failed to read line");

    let account = projector::project(account_id.trim());

    if account.is_none() {
        println!("Invalid Account");
        return;
    }

    let account = account.unwrap();

    let menu = menu(vec![

        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("Select Operation"),
        label("-----------------------"),

        // button:
        //  exit the menu
        button("Withdraw"),
        button("Deposit"),
    ]);
    run(&menu);

    let operation = mut_menu(&menu).selected_item_name().to_string();

    println!("operation {}", operation);

    match operation.as_str() {
        "Withdraw" =>     handle_withdraw(&account),
        "Deposit" =>     println!("lets with"),
        _ => println!("Invalid operation"),
    }
}

fn handle_withdraw (account: &Account) {

    let mut amount: Option<u32> = None;

    while amount.is_none() {
        let mut input = String::new();

        println!("Amout: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        if let Ok(value) = input.trim().parse::<u32>() {
            amount = Some(value);
        }
    }

    let amount = amount.unwrap();

    let data = Withdraw {
        account_id: "123".to_string(),
        amount,
        time: Utc::now().to_string(),
        sequence: account.sequence,
    }.as_json();

    if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
        println!("Failed producing messages: {}", e);
    }

    let data = Withdraw {
        account_id: "123".to_string(),
        amount,
        time: Utc::now().to_string(),
        sequence: account.sequence,
    }.as_json();

    if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
        println!("Failed producing messages: {}", e);
    }
}
