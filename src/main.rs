use clap::{Args, Parser, Subcommand, ValueEnum};

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
    position: u8,
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

    match &cli.command.unwrap_or(TodoCommands::List) {
        TodoCommands::List => {
            println!("List\nof\ntodos");
        }
        TodoCommands::Add(AddArgs {
            description,
            priority,
        }) => {
            println!("Adding todo {:?} with priority {:?}", description, priority)
        }
        TodoCommands::Remove(RemoveArgs { position }) => {
            println!("Removing todo {:?}", position)
        }
    }
}
