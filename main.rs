#![allow(dead_code)]

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: u32, holder_name: &str) -> BankAccount {
        BankAccount {
            account_number,
            holder_name: holder_name.to_string(),
            balance: 0.0,
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Deposit amount must be greater than 0".to_string())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds for withdrawal".to_string())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    
    let mut account1 = BankAccount::new(12345, "Alice");
    let mut account2 = BankAccount::new(67890, "Bob");

    
    match account1.deposit(100.0) {
        Ok(_) => println!("Deposit to account1 successful!"),
        Err(err) => println!("Error: {}", err),
    }

    
    match account2.withdraw(50.0) {
        Ok(_) => println!("Withdraw from account2 successful!"),
        Err(err) => println!("Error: {}", err),
    }

    
    println!("Balance of account1: {}", account1.balance());
    println!("Balance of account2: {}", account2.balance());
}
