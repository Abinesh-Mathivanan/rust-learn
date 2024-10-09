use std::collections::HashMap;
use std::io::{self, Write};

struct Database {
    storage: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            storage: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    pub fn remove_data(&mut self, key: &str) {
        self.storage.remove(key);
    }

    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }
}

fn main() {
    let mut red_cln = Database::new();

    loop {
        print!("Enter the commands: ");
        io::stdout().flush().unwrap();

        let mut cmd_input = String::new();
        io::stdin()
            .read_line(&mut cmd_input)
            .expect("Failed to parse input");
        let cmd_input = cmd_input.trim();

        if cmd_input.eq_ignore_ascii_case("EXIT") {
            break;
        }

        let cmd_parts: Vec<&str> = cmd_input.split_whitespace().collect();
        match cmd_parts.as_slice() {
            ["SET", key, value] => {
                red_cln.add_data(key.to_string(), value.to_string());
                println!("Stored {} -> {}", key, value);
            }
            ["GET", key] => match red_cln.get_data(key) {
                Some(value) => println!("{} -> {}", key, value),
                None => println!("Key {} not found", key),
            },
            ["DELETE", key] => {
                red_cln.remove_data(key);
                println!("Deleted {}", key);
            }
            _ => {
                println!("Invalid Command");
            }
        }
    }

    println!("Exiting...");
}
