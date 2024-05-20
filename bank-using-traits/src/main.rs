// Step 1: Define the Account Trait
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn get_balance(&self) -> f64;
}

// Step 2: Create Structs for Different Types of Accounts
struct CheckingAccount {
    balance: f64,
    overdraft_limit: f64,
}

struct SavingsAccount {
    balance: f64,
    interest_rate: f64,
}

// Step 3: Implement the Account Trait for CheckingAccount
impl Account for CheckingAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance + self.overdraft_limit >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

// Step 3: Implement the Account Trait for SavingsAccount
impl Account for SavingsAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

// Step 4: Write the main Function
fn main() {
    // Create a CheckingAccount
    let mut checking_account = CheckingAccount {
        balance: 1000.0,
        overdraft_limit: 500.0,
    };

    // Create a SavingsAccount
    let mut savings_account = SavingsAccount {
        balance: 2000.0,
        interest_rate: 0.03,
    };

    // Deposit money into CheckingAccount
    checking_account.deposit(200.0);
    println!("CheckingAccount balance: ${}", checking_account.get_balance());

    // Withdraw money from CheckingAccount
    match checking_account.withdraw(1500.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(e) => println!("Failed to withdraw: {}", e),
    }
    println!("CheckingAccount balance: ${}", checking_account.get_balance());

    // Deposit money into SavingsAccount
    savings_account.deposit(500.0);
    println!("SavingsAccount balance: ${}", savings_account.get_balance());

    // Withdraw money from SavingsAccount
    match savings_account.withdraw(1000.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(e) => println!("Failed to withdraw: {}", e),
    }
    println!("SavingsAccount balance: ${}", savings_account.get_balance());
}
