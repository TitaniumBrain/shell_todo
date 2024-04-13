use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self};
use std::{fs, fs::File};

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

pub fn list_tasks() -> Result<(), &'static str> {
    let tasks = get_tasks()?;

    for task in tasks {
        println!("{0:<12}{1}", task.priority, task.description)
    }
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
