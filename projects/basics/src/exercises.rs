fn hello() {
    println!("exercises!");
}

// Count each letter in a given string.
pub mod strings {
    // inner module has its own use statement.
    use std::collections::HashMap;

    pub fn hello_str() {
        println!("Hello, string exercises!");
    }

    pub fn letter_frequencies(s: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();
        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        freq
    }
}

pub mod shorts {
    // inner module has its own use statement.
    use std::collections::HashMap;

    pub struct BankAccount {
        pub name: String,
        pub balance: f64,
    }

    impl BankAccount {
        fn new(name: String, balance: f64) -> Self {
            BankAccount { name, balance }
        }

        // borrow self - mutator
        // remember, it should be called on a mutable variable
        fn deposit(&mut self, amount: f64) {
            println!("Deposit: {}", amount);
            self.balance += amount;
        }

        // borrow self - mutator
        fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
            if (amount > self.balance) {
                return Err("Insufficient funds".to_string());
            }

            self.balance -= amount;
            Ok(self.balance)
        }

        // borrow self - no mutation
        fn check_balance(&self) {
            println!("Account Balance: {}", self.balance)
        }
    }

    pub fn piggybank_1() {
        // lira: 1, 0.5, 0.25, 0.1
        let assets = (1, 2, 3, 4);
        let total_value = assets.0 * 100 + assets.1 * 50 + assets.2 * 25 + assets.3 * 10;
        println!("Total: {}", total_value);
    }

    pub fn piggybank_2() {
        let mut assets = HashMap::new();
        assets.insert(String::from("pennies"), 5);
        assets.insert(String::from("nickels"), 4); // 5 cents
        assets.insert(String::from("dimes"), 3); // 10 cents
        assets.insert(String::from("quarters"), 2); // 25 cents

        println!("{:?}", assets); // {"pennies": 5, "quarters": 4, "nickels": 3, "dimes": 2}

        let total_value = 0;
        println!("Total: {}", total_value);
    }

    pub fn bank_account_demo() {
        println!("bank!");

        // we will mutate this instance with methods, so we define it as mutable.
        let mut account1 = BankAccount::new(String::from("MK"), 0.0);

        account1.check_balance();

        // mutate account1
        account1.deposit(10.0);

        account1.withdraw(5.0);

        match account1.withdraw(15.0) {
            Ok(bal) => println!("Withdrawal successful"),
            Err(msg) => println!("Withdrawal failed: {}", msg),
        }

        account1.check_balance();
    }
}
