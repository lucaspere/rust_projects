#[derive(Debug)] // Para poder imprimir
pub struct BankAccount {
    pub id: u32,
    pub balance: i64,
    pub transaction_log: Vec<String>,
}

impl BankAccount {
    pub fn new(id: u32, initial_balance: i64) -> Self {
        BankAccount {
            id,
            balance: initial_balance,
            transaction_log: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: i64, actor: &str) {
        if amount > 0 {
            self.balance += amount;
            self.transaction_log.push(format!(
                "{} deposited: {}. New balance: {}",
                actor, amount, self.balance
            ));
        }
    }

    pub fn withdraw(&mut self, amount: i64, actor: &str) -> bool {
        if amount > 0 && self.balance >= amount {
            self.balance -= amount;
            self.transaction_log.push(format!(
                "{} withdrew: {}. New balance: {}",
                actor, amount, self.balance
            ));
            true
        } else {
            self.transaction_log.push(format!(
                "{} failed to withdraw: {}. Insufficient funds or invalid amount. Balance: {}",
                actor, amount, self.balance
            ));
            false
        }
    }
}
