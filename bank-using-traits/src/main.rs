// Define the Account trait with deposit, withdraw, and balance methods
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Implement the Account trait for BankAccount struct
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err("Deposit amount cannot be negative".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err("Withdrawal amount cannot be negative".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: "SBIN00034".to_string(),
        holder_name: "Mishal Turkane".to_string(),
        balance: 100.0,
    };


    // Deposit money into account1
    match account1.deposit(50.0) {
        Ok(_) => println!("Deposit successful"),
        Err(err) => println!("Deposit failed: {}", err),
    }

    // Print balances of both accounts
    println!("Account 1 Balance: {}", account1.balance());
}