use std::io;
use std::vec::Vec;

// Struct to represent a task
struct Task {
    description: String,
    done: bool,
}

impl Task {
    // Creating a new task
    fn new(description: String) -> Task {
        Task {
            description,
            done: false,
        }
    }

    // Method to mark a task as done
    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Task Manager");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Done");
        println!("4. Quit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input");
                let task = Task::new(description.trim().to_string());
                tasks.push(task);
                println!("Task added.");
            }
            2 => {
                println!("Tasks:");
                for (index, task) in tasks.iter().enumerate() {
                    let status = if task.done { "Done" } else { "Not Done" };
                    println!("{}. [{}] {}", index + 1, status, task.description);
                }
            }
            3 => {
                println!("Enter task number to mark as done:");
                let mut task_number = String::new();
                io::stdin()
                    .read_line(&mut task_number)
                    .expect("Failed to read input");
                let task_number: usize = match task_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid task number.");
                        continue;
                    }
                };

                if task_number <= 0 || task_number > tasks.len() {
                    println!("Invalid task number. Please enter a valid task number.");
                } else {
                    let task = &mut tasks[task_number - 1];
                    task.mark_done();
                    println!("Task marked as done.");
                }
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }
}