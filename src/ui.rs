use crate::chain::{Blockchain, Transaction};
use std::sync::{Arc, Mutex};
use std::io::{self, Write};

pub fn start_ui(blockchain: Arc<Mutex<Blockchain>>) -> Result<(), io::Error> {
    loop {
        println!("\nWelcome to the ZephyrChain Interface!");
        println!("1. Retrieve Transaction History - View all transactions associated with an address.");
        println!("2. Check Balance - Check the current balance of an address.");
        println!("3. Exit - Exit the interface.");
        print!("Please enter your choice: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => retrieve_transaction_history_ui(&blockchain),
            "2" => check_balance_ui(&blockchain),
            "3" => break,
            _ => {
                println!("Invalid choice, please try again.");
                continue;
            },
        }
    }
    Ok(())
}

fn check_balance_ui(blockchain: &Arc<Mutex<Blockchain>>) {
    print!("Enter address to check balance: ");
    io::stdout().flush().unwrap();
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();
    let address = address.trim();

    let blockchain_lock = blockchain.lock();
    match blockchain_lock {
        Ok(blockchain) => {
            let balance = blockchain.retrieve_balance(address);
            match balance {
                Ok(balance) => println!("Balance for {}: {}", address, balance),
                Err(_) => println!("Failed to retrieve balance for {}. Please ensure the address is correct.", address),
            }
        },
        Err(_) => println!("Failed to acquire blockchain lock. Please try again."),
    }
}