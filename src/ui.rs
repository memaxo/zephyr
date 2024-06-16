use crate::chain::{Blockchain, Transaction};
use std::sync::{Arc, Mutex};
use std::io::{self, Write};

pub fn start_ui(blockchain: Arc<Mutex<Blockchain>>) -> Result<(), io::Error> {
    loop {
        println!("\nWelcome to the ZephyrChain Interface!");
        println!("1. Retrieve Transaction History - View all transactions associated with an address.");
        println!("2. Check Balance - Check the current balance of an address.");
        println!("3. Submit Task - Submit a new task to the marketplace.");
        println!("4. Browse Tasks - Browse and filter tasks.");
        println!("5. Node Dashboard - Manage bids and track performance.");
        println!("6. Exit - Exit the interface.");
        print!("Please enter your choice: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => retrieve_transaction_history_ui(&blockchain),
            "2" => check_balance_ui(&blockchain),
            "3" => submit_task_ui(&blockchain),
            "4" => browse_tasks_ui(&blockchain),
            "5" => node_dashboard_ui(&blockchain),
            "6" => break,
            _ => {
                println!("Invalid choice, please try again.");
                continue;
            },
        }
    }
    Ok(())
}

fn submit_task_ui(blockchain: &Arc<Mutex<Blockchain>>) {
    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim().to_string();

    print!("Enter task resources (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut resources = String::new();
    io::stdin().read_line(&mut resources).unwrap();
    let resources: Vec<String> = resources.trim().split(',').map(|s| s.trim().to_string()).collect();

    print!("Enter task reward: ");
    io::stdout().flush().unwrap();
    let mut reward = String::new();
    io::stdin().read_line(&mut reward).unwrap();
    let reward: u64 = reward.trim().parse().unwrap();

    print!("Enter task deadline (YYYY-MM-DDTHH:MM:SSZ): ");
    io::stdout().flush().unwrap();
    let mut deadline = String::new();
    io::stdin().read_line(&mut deadline).unwrap();
    let deadline: DateTime<Utc> = deadline.trim().parse().unwrap();

    print!("Enter your address: ");
    io::stdout().flush().unwrap();
    let mut creator = String::new();
    io::stdin().read_line(&mut creator).unwrap();
    let creator = creator.trim().to_string();

    let task = Task::new(0, description, resources, reward, deadline, creator);

    let blockchain_lock = blockchain.lock();
    match blockchain_lock {
        Ok(mut blockchain) => {
            match blockchain.submit_task(task) {
                Ok(_) => println!("Task submitted successfully."),
                Err(e) => println!("Failed to submit task: {}", e),
            }
        },
        Err(_) => println!("Failed to acquire blockchain lock. Please try again."),
    }
}

fn browse_tasks_ui(blockchain: &Arc<Mutex<Blockchain>>) {
    let blockchain_lock = blockchain.lock();
    match blockchain_lock {
        Ok(blockchain) => {
            let tasks = blockchain.get_all_tasks();
            for task in tasks {
                println!("Task ID: {}", task.id);
                println!("Description: {}", task.description);
                println!("Resources: {:?}", task.resources);
                println!("Reward: {}", task.reward);
                println!("Deadline: {}", task.deadline);
                println!("Creator: {}", task.creator);
                println!("-----------------------------");
            }
        },
        Err(_) => println!("Failed to acquire blockchain lock. Please try again."),
    }
}

fn node_dashboard_ui(blockchain: &Arc<Mutex<Blockchain>>) {
    print!("Enter your node address: ");
    io::stdout().flush().unwrap();
    let mut node_address = String::new();
    io::stdin().read_line(&mut node_address).unwrap();
    let node_address = node_address.trim().to_string();

    let blockchain_lock = blockchain.lock();
    match blockchain_lock {
        Ok(blockchain) => {
            let bids = blockchain.get_bids_by_node(&node_address);
            for bid in bids {
                println!("Task ID: {}", bid.task_id);
                println!("Proposed Time: {}", bid.proposed_time);
                println!("Proposed Reward: {}", bid.proposed_reward);
                println!("Proof of Capability: {}", bid.proof_of_capability);
                println!("-----------------------------");
            }

            let performance = blockchain.get_node_performance(&node_address);
            println!("Node Performance: {}", performance);
        },
        Err(_) => println!("Failed to acquire blockchain lock. Please try again."),
    }
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
