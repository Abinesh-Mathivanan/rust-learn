use std::io::{self, Write};

struct Tasks {
    id: usize,
    title: String,
    description: String,
}

struct Todolist {
    tasks: Vec<Tasks>,
}


impl Todolist {
    pub fn new() -> Todolist {
        Todolist {
            tasks: Vec::new(),
        }
    }
    pub fn insert_tasks(&mut self, title: String, description: String) {
        self.tasks.push(Tasks { id: (self.tasks.len()+1), title: (title.to_string()), description: (description.to_string()) });
    }
    pub fn delete_tasks(&mut self, id: usize){
        self.tasks.remove(id);
    }
    pub fn list_tasks(&mut self){
        for i in &self.tasks{
            println!("{} {}", i.title, i.description);
        }
    }
}   

fn main(){
    let mut new_to_do_list = Todolist::new();
    
    loop {
        println!{"1. Add task"};
        println!("2. Remove task");
        println!("3. List tasks");
        println!("4. Exit");
        println!("Enter your choice:");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to Parse");
        let choice = choice.trim().parse::<i32>();

        match choice {
            Ok(1) => {
                println!("Enter task title and description:");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                let mut description = String::new();
                io::stdin().read_line(&mut title).expect("Failed to parse title");
                io::stdin().read_line(&mut description).expect("Failed to parse description");
                new_to_do_list.insert_tasks(title, description);
            }
            Ok(2) => {
                println!("Enter task id");
                io::stdout().flush().unwrap();
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to parse Task ID");
                if let Ok(task_id) = task_id.trim().parse::<usize>() {
                    new_to_do_list.delete_tasks(task_id);
                } else {
                    println!("Couldn't find Task ID");
                }
            }
            Ok(3) => {
                println!("The tasks are");
                new_to_do_list.list_tasks();
            }
            Ok(4) => {
                println!("Breaking");
                break;
            }
            _ => {
                println!("Invalid Choice. Try Again");
            }
        }
    }
}