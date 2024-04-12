use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<TodoCommands>,
}
#[derive(Subcommand, Debug)]
enum TodoCommands {
    List,
    Add(AddArgs),
    Remove(RemoveArgs),
}

#[derive(Args, Debug)]
struct AddArgs {
    description: String,
    #[arg(short, long, value_enum, default_value_t=Priority::Normal)]
    priority: Priority,
}

#[derive(Args, Debug)]
struct RemoveArgs {
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
