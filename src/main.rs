use std::{io::Write};

fn flag_invalid_input(input: &str) {
    println!("Invalid input: {}\n", input);
    print_help();
}

fn print_help() {
    println!("----- HELP -----");
    println!(
        r"Commands are not case sensitive

DEBIT <amount>: Create a debit record
CREDIT <amount>: Create a credit record
BALANCE: Check the balance
LIST: List records
QUIT: End the program
HELP: Display this help message"
    );
}

/// Helper function to add a record to the list
fn add_record(records: &mut Vec<f32>, amount: f32) {
    println!("Adding entry: {}", amount);
    records.push(amount)
}

fn create_debit_record(records: &mut Vec<f32>, amount: f32) {
    add_record(records, -amount);
}

fn create_credit_record(records: &mut Vec<f32>, amount: f32) {
    add_record(records, amount);
}

fn get_balance(records: &[f32]) -> f32 {
    records.iter().sum()
}

/// Helper function to format records in a user-readable format
fn format_records_to_string(records: &[f32]) -> String {
    let mut output = String::new();
    for entry in records {
        let t = if entry < &0.0 { "Debit\t" } else {"Credit\t"};
        let amount = entry.abs().to_string();
        output.push_str(&(t.to_owned() + &amount));
        output.push('\n');
    };
    output
}

fn main() {
    println!("Welcome to the Bookkeeping App!\n");
    let mut records: Vec<f32> = vec![0.0; 0];
    print_help();
    println!("\nHappy bookkeeping!\n");

    let mut input = String::new();
    loop {
        // Collect user input
        print!(">>> ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line from stdin");
        input = input.trim().to_ascii_lowercase();

        let tokens: Vec<&str> = input.split_whitespace().collect();
        let command = tokens.get(0).copied();
        let amount = tokens.get(1).and_then(|s| s.parse::<f32>().ok());

        match command {
            None => {}
            Some("quit") => break,
            Some("help") => print_help(),
            Some("list") => println!("{}", format_records_to_string(&records)),
            Some("balance") => println!("{}", get_balance(&records)),
            Some("credit") => match amount {
                Some(amount) => create_credit_record(&mut records, amount),
                None => flag_invalid_input(&input),
            },
            Some("debit") => match amount {
                Some(amount) => create_debit_record(&mut records, amount),
                None => flag_invalid_input(&input),
            },
            _ => flag_invalid_input(&input),
        };

        // Clear the input buffer for the next input
        input.clear();
    }

    println!("\nGoodbye! See you next time.")
}
