mod bank_accounts;
use bank_accounts::BankAccount;

fn main() {
    let mut account = BankAccount::new(500.0);
    account.deposit(200.0);
    account.withdraw(100.0);

    println!("Final balance: {}", account.balance());
}
