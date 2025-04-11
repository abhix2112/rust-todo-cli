use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Todostatus {
    Completed,
    NotCompleted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    status: Todostatus,
    priority: Priority,
    created: String,
}

fn load_todos() -> Vec<Todo> {
    if let Ok(encoded) = fs::read_to_string("todo.json") {
        if let Ok(decoded) = base64::decode(&encoded) {
            if let Ok(json_string) = String::from_utf8(decoded) {
                if let Ok(todos) = serde_json::from_str(&json_string) {
                    return todos;
                }
            }
        }
    }
    Vec::new()
}

fn save_todos(todos: &Vec<Todo>) {
    if let Ok(json_string) = serde_json::to_string(&todos) {
        let encoded = base64::encode(json_string);
        fs::write("todo.json", encoded).expect("Failed to write to file");
    }
}

fn main() {
    let mut todos = load_todos();

    loop {
        println!("\n📋 Welcome to the Todo List");
        println!("1. Add Todo");
        println!("2. View Todos");
        println!("3. Edit Todo");
        println!("4. Delete Todo");
        println!("5. Mark Completed/Not Completed");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

                println!("Enter Todo title: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                println!("Enter Todo description: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                println!("Enter status (0 for Completed, 1 for Not Completed): ");
                let mut status = String::new();
                io::stdin().read_line(&mut status).unwrap();
                let status = match status.trim() {
                    "0" => Todostatus::Completed,
                    "1" => Todostatus::NotCompleted,
                    _ => {
                        println!("❌ Invalid status value.");
                        continue;
                    }
                };

                println!("Enter priority (1: High, 2: Medium, 3: Low): ");
                let mut priority = String::new();
                io::stdin().read_line(&mut priority).unwrap();
                let priority = match priority.trim() {
                    "1" => Priority::High,
                    "2" => Priority::Medium,
                    "3" => Priority::Low,
                    _ => {
                        println!("❌ Invalid priority value.");
                        continue;
                    }
                };

                let created = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                todos.push(Todo {
                    id,
                    title: title.trim().to_string(),
                    description: description.trim().to_string(),
                    status,
                    priority,
                    created,
                });

                save_todos(&todos);
                println!("✅ Todo added successfully!");
            }

            2 => {
                if todos.is_empty() {
                    println!("📭 No todos found.");
                } else {
                    println!("\n📋 Todos:");
                    for todo in &todos {
                        println!("{:?}", todo);
                    }
                }
            }

            3 => {
                println!("Enter Todo ID to edit: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("❌ Invalid input.");
                        continue;
                    }
                };

                if let Some(index) = todos.iter().position(|t| t.id == id) {
                    println!("Enter Updated title: ");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).unwrap();

                    println!("Enter Updated description: ");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).unwrap();

                    println!("Enter Updated status (0: Completed, 1: Not Completed): ");
                    let mut status = String::new();
                    io::stdin().read_line(&mut status).unwrap();
                    let status = match status.trim() {
                        "0" => Todostatus::Completed,
                        "1" => Todostatus::NotCompleted,
                        _ => {
                            println!("❌ Invalid status.");
                            continue;
                        }
                    };

                    println!("Enter Updated priority (1: High, 2: Medium, 3: Low): ");
                    let mut priority = String::new();
                    io::stdin().read_line(&mut priority).unwrap();
                    let priority = match priority.trim() {
                        "1" => Priority::High,
                        "2" => Priority::Medium,
                        "3" => Priority::Low,
                        _ => {
                            println!("❌ Invalid priority.");
                            continue;
                        }
                    };

                    todos[index].title = title.trim().to_string();
                    todos[index].description = description.trim().to_string();
                    todos[index].status = status;
                    todos[index].priority = priority;

                    save_todos(&todos);
                    println!("✅ Todo updated.");
                } else {
                    println!("❌ Todo ID not found.");
                }
            }

            4 => {
                println!("Enter Todo ID to delete: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("❌ Invalid input.");
                        continue;
                    }
                };

                if let Some(index) = todos.iter().position(|t| t.id == id) {
                    todos.remove(index);
                    save_todos(&todos);
                    println!("🗑️ Todo deleted.");
                } else {
                    println!("❌ Todo ID not found.");
                }
            }

            5 => {
                println!("Enter Todo ID to update status: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("❌ Invalid input.");
                        continue;
                    }
                };

                if let Some(index) = todos.iter().position(|t| t.id == id) {
                    println!("Enter Updated status (0: Completed, 1: Not Completed): ");
                    let mut status = String::new();
                    io::stdin().read_line(&mut status).unwrap();
                    let status = match status.trim() {
                        "0" => Todostatus::Completed,
                        "1" => Todostatus::NotCompleted,
                        _ => {
                            println!("❌ Invalid status.");
                            continue;
                        }
                    };

                    todos[index].status = status;
                    save_todos(&todos);
                    println!("✅ Todo status updated.");
                } else {
                    println!("❌ Todo ID not found.");
                }
            }

            6 => {
                println!("👋 Exiting...");
                break;
            }

            _ => println!("❌ Invalid option."),
        }
    }
}
