use clap::{Args, Parser, Subcommand, ValueEnum};
use shell_todo::{add_task, list_tasks, remove_task, Task};

#[derive(Parser, Debug)]
#[command(version, about)]
/// Simple todo list manager
struct Cli {
    #[command(subcommand)]
    command: Option<TodoCommands>,
}
#[derive(Subcommand, Debug)]
enum TodoCommands {
    /// List all pending tasks
    List,
    /// Add a new task
    Add(AddArgs),
    /// Remove a task
    Remove(RemoveArgs),
}

#[derive(Args, Debug)]
struct AddArgs {
    /// The description of the task you want to add to your list
    description: String,
    /// Set the priority level of the task
    #[arg(short, long, value_enum, default_value_t=Priority::Normal)]
    priority: Priority,
}

#[derive(Args, Debug)]
struct RemoveArgs {
    /// Position of the task to remove
    position: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

fn main() {
    let cli = Cli::parse();

    if let Err(result) = match cli.command.unwrap_or(TodoCommands::List) {
        TodoCommands::List => list_tasks(),
        TodoCommands::Add(AddArgs {
            description,
            priority,
        }) => {
            let priority = match priority {
                Priority::Low => 0,
                Priority::Normal => 1,
                Priority::High => 2,
                Priority::Urgent => 3,
            };
            add_task(Task {
                description,
                priority,
            })
        }
        TodoCommands::Remove(RemoveArgs { position }) => remove_task(position),
    } {
        eprintln!("{result}")
    };
}
