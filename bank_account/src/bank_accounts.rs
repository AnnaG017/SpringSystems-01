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
         if amount < 0.0 {
            return;
        }
        self.balance += amount;
    }

    

    pub fn withdraw(&mut self, amount: f64) {
         if amount < 0.0 {
            return;
        }
        if amount > self.balance {
            return;
        }
        self.balance -= amount;
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
    fn test_negative_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0); 
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_negative_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0); 
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_overdraft_attempt() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0); 
        assert_eq!(account.balance(), 100.0);
    }
    // Add more tests here
}