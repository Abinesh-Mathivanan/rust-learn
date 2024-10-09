struct Account {
    id: i32,
    name: String,
    balance: f64,
}

struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Account {
    pub fn new(id: i32, name: &str, balance: f64) -> Self{
        Self {
            id,
            name: name.to_string(),
            balance,
        }
    }
    pub fn deposit(&mut self, amount: f64){
        self.balance += amount;
    }
    pub fn withdraw(&mut self, amount: f64) -> bool{
        if self.balance >= amount {
            self.balance -= amount;
            true
        }
        else{
            false
        }
    }
    pub fn display_balance(&self){
        println!("{}",self.balance);
    }

}

impl Bank {
    pub fn new(name: &str) -> Self{
        Self {
            name: name.to_string(),
            accounts: Vec::new(),
        }
    }
    pub fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }
    pub fn deposit_account(&mut self, account_id: i32, amount: f64){
        if let Some(account) = self.accounts.iter_mut().find(|acc| acc.id == account_id){
            account.deposit(amount);
            println!("Amount {} deposited to {}", amount, account_id);
        }
        else {
            println!("Account deposit failed");
        }
    }
    pub fn withdraw_account(&mut self, account_id: i32, amount: f64){
        if let Some(account) = self.accounts.iter_mut().find(|acc| acc.id == account_id){
            if account.balance >= amount {
                account.balance -= amount;
            } else {
                println!("Insufficient Balance!");
            }
            println!("Withdraw {} from account {} is successful", amount, account_id);
        } else {
            println!("Couldn't find account");
        }
    }
    pub fn transfer_account(&mut self, from_account_id: i32, to_account_id: i32, amount: f64) {
        if from_account_id == to_account_id {
            println!("Transfer to the same account is not allowed.");
            return;
        }
        let from_index = self.accounts.iter().position(|acc| acc.id == from_account_id);
        let to_index = self.accounts.iter().position(|acc| acc.id == to_account_id);
    
        if let (Some(from_index), Some(to_index)) = (from_index, to_index) {
            let (from_account, to_account) = if from_index < to_index {
                let (left, right) = self.accounts.split_at_mut(to_index);
                (&mut left[from_index], &mut right[0])
            } else {
                let (left, right) = self.accounts.split_at_mut(from_index);
                (&mut right[0], &mut left[to_index])
            };
    
            if from_account.withdraw(amount) {
                to_account.deposit(amount);
                println!(
                    "{:.2} deposited from account {} to account {}",
                    amount, from_account_id, to_account_id
                );
            } else {
                println!("Failed to withdraw from account {}: insufficient funds.", from_account_id);
            }
        } else {
            println!("One or both accounts not found!");
        }
    }    
    pub fn account_display(&mut self, account_id: i32){
        if let Some(account) = self.accounts.iter_mut().find(|acc| acc.id == account_id){
            account.display_balance();
        }
        else {
            println!("Unable to display balance");
        }
    }
}

fn main(){
    let mut new_bank = Bank::new("Beens Bank");
    let account_one = Account::new(1, "Abinesh", 8000.0);
    let account_two = Account::new(2, "Raj", 10000.0);
    new_bank.add_account(account_one);
    new_bank.add_account(account_two);

    new_bank.deposit_account(1, 10000.0);
    new_bank.deposit_account(2, 1500.0);
    new_bank.withdraw_account(1, 5000.0);
    new_bank.transfer_account(1, 2, 3000.0);
    new_bank.account_display(1);
}
