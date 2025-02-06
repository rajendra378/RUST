use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: u32,
    description: String, // Fixed typo: `desciption` -> `description`
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\nTask Manager");
        println!("1. Add Task");
        println!("2. Mark Task as Completed");
        println!("3. Remove Task");
        println!("4. Display Tasks");
        println!("5. Clear Tasks");
        println!("6. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                // Add Task
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input");

                tasks.push(Task {
                    id: next_id,
                    description: description.trim().to_string(), // Fixed typo
                    completed: false,
                });
                next_id += 1;
                println!("Task added!");
            }
            Ok(2) => {
                // Mark Task as Completed
                print!("Enter task ID to mark as completed: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read input");

                if let Ok(id) = id_input.trim().parse::<u32>() {
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                        task.completed = true;
                        println!("Task {} marked as completed!", id);
                    } else {
                        println!("Task not found!");
                    }
                } else {
                    println!("Invalid ID!");
                }
            }
            Ok(3) => {
                // Remove Task
                print!("Enter the task ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read input");

                if let Ok(id) = id_input.trim().parse::<u32>() {
                    if let Some(index) = tasks.iter().position(|t| t.id == id) {
                        tasks.remove(index);
                        println!("Task {} removed!", id);
                    } else {
                        println!("Task not found!"); // Fixed typo: `esle` -> `else`
                    }
                } else {
                    println!("Invalid ID!");
                }
            }
            Ok(4) => {
                // Display all Tasks
                if tasks.is_empty() {
                    println!("No tasks found!");
                } else {
                    println!("Tasks:");
                    for task in &tasks {
                        println!(
                            "ID: {}, Description: {}, Completed: {}",
                            task.id, task.description, task.completed
                        );
                    }
                }
            }
            Ok(5) => {
                // Clear Tasks
                tasks.clear();
                println!("All tasks cleared!");
            }
            Ok(6) => {
                // Exit
                println!("Exiting Task Manager...");
                break;
            }
            _ => {
                println!("Invalid option! Please choose a number between 1 and 6."); // Fixed extra semicolon
            }
        }
    }
}