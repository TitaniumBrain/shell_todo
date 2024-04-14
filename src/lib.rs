use colored::Colorize;
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self};
use std::{fs, fs::File};
use tabled::builder::Builder;
use tabled::settings::{object::Columns, Style, Width};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub priority: u8,
}

/// Reads the tasks file, creating it if it exists
fn get_tasks() -> Result<Vec<Task>, &'static str> {
    let mut data_dir = match dirs::data_dir() {
        Some(data_dir) => data_dir,
        None => {
            return Err("Cannot determine user's data directory.");
        }
    };

    data_dir.push("shell_todo/tasks.json");

    match fs::read_to_string(&data_dir) {
        Ok(data_string) => {
            serde_json::from_str::<Vec<Task>>(&data_string).map_err(|_| "Could not create data dir")
        }

        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                let parent_path = data_dir
                    .parent()
                    .ok_or_else(|| "Could not create data dir")?;

                fs::create_dir_all(parent_path).map_err(|_| "Could not create data dir")?;

                match File::create(&data_dir) {
                    Ok(_) => return Ok(Vec::new()),
                    Err(error) => {
                        eprintln!("{:#?}", error);
                        return Err("Could not create data file.");
                    }
                }
            }
            _ => Err("Could not read file."),
        },
    }
}

/// Print all the tasks in a colour coded table
pub fn list_tasks(no_colour: bool) -> Result<(), &'static str> {
    let mut tasks = get_tasks()?;
    tasks.sort_by(|t1, t2| t2.priority.cmp(&t1.priority));

    colored::control::set_override(!no_colour);

    let mut table_builder = Builder::default();
    for (index, task) in tasks.iter().enumerate() {
        let priority = match task.priority {
            0 => "low".dimmed(),
            1 => "normal".white(),
            2 => "high".yellow(),
            3 => "urgent".red().bold(),
            _ => "".clear(),
        };
        table_builder.push_record([
            format!("{index}"),
            format!("{0}", priority),
            format!("{0}", task.description),
        ])
    }
    // build table and set styling
    let mut table = table_builder.build();
    table.with(Style::modern());
    table
        .modify(Columns::single(1), Width::increase(8))
        .modify(Columns::last(), Width::wrap(60));

    println!("{table}");
    Ok(())
}

/// Adds a task to the vec and saves it to file
pub fn add_task(task: Task) -> Result<(), &'static str> {
    let mut tasks = get_tasks()?;
    tasks.push(task);

    // serialize
    let json_string =
        serde_json::to_string_pretty(&tasks).map_err(|_| "Could not serialize task")?;

    // get data directory
    let mut data_dir = match dirs::data_dir() {
        Some(data_dir) => data_dir,
        None => {
            return Err("Cannot determine user's data directory.");
        }
    };
    data_dir.push("shell_todo/tasks.json");

    // serialize and save
    fs::write(data_dir, json_string).map_err(|_| "Could not serialize task")
}

/// Removes a task from the list
pub fn remove_task(position: usize) -> Result<(), &'static str> {
    let mut tasks = get_tasks()?;

    // remove task
    if position < tasks.len() {
        tasks.remove(position);
    }

    // serialize
    let json_string =
        serde_json::to_string_pretty(&tasks).map_err(|_| "Could not serialize task")?;

    // get data directory
    let mut data_dir = match dirs::data_dir() {
        Some(data_dir) => data_dir,
        None => {
            return Err("Cannot determine user's data directory.");
        }
    };
    data_dir.push("shell_todo/tasks.json");

    // serialize and save
    fs::write(data_dir, json_string).map_err(|_| "Could not serialize task")
}
