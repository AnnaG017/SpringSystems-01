#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {balance: initial_balance,}// creating a new BankAccount with the given starting balance
        // Implement this method
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        // Implement this method
    }

    pub fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");//if there is no more will print this
        }
        // Implement this method
    }

    pub fn balance(&self) -> f64 {
        //return the current balance
        self.balance
        // Implement this method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(300.0); // will start with $300
        assert_eq!(account.balance(), 300.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(200.0); // new account with $200
        account.deposit(100.0); // deposit $100
        assert_eq!(account.balance(), 300.0); // should now be $300
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(200.0); // start with $200
        account.withdraw(50.50); // withdraw $80
        assert_eq!(account.balance(), 149.5); // should now be $120
    }

    // Add more tests here
}