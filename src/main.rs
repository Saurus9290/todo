use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

/// Represents a single task in the todo list.
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

/// Command-line arguments for the todo app.
#[derive(Parser)]
#[command(name = "Todo List", version = "1.0", about = "A simple CLI todo list.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands for the todo app.
#[derive(Subcommand)]
enum Commands {
    /// Add a new task.
    Add { description: String },

    /// List all tasks.
    List {},

    /// Mark a task as completed.
    Complete { id: usize },

    /// Delete a task.
    Delete { id: usize },
}

const FILE_PATH: &str = "tasks.json";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => add_task(description),
        Commands::List {} => list_tasks(),
        Commands::Complete { id } => complete_task(id),
        Commands::Delete { id } => delete_task(id),
    }
}

/// Load tasks from the JSON file, or return an empty list if the file doesn't exist.
fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

/// Save tasks to the JSON file.
fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks.");
    let mut file = File::create(FILE_PATH).expect("Failed to create tasks file.");
    file.write_all(data.as_bytes())
        .expect("Failed to write to tasks file.");
}

/// Add a new task to the list.
fn add_task(description: String) {
    let mut tasks = load_tasks();
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        description,
        completed: false,
    });
    save_tasks(&tasks);
    println!("Task added successfully.");
}

/// List all tasks with their statuses.
fn list_tasks() {
    let tasks = load_tasks();
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for task in &tasks {
            println!(
                "[{}] {} - {}",
                if task.completed { "x" } else { " " },
                task.id,
                task.description
            );
        }
    }
}

/// Mark a task as completed.
fn complete_task(id: usize) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        save_tasks(&tasks);
        println!("Task {} marked as completed.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}

/// Delete a task from the list.
fn delete_task(id: usize) {
    let mut tasks = load_tasks();
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        save_tasks(&tasks);
        println!("Task {} deleted successfully.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}
