use extendr_api::prelude::*;
use serde::{Serialize, Deserialize};
use home::home_dir;
use std::path;
use std::io::{Result, Write};
use std::fs::OpenOptions;

/// Add a task
/// @export
#[extendr]
fn add_task(time: String, command: String, schedule_type: String) -> &'static str {
    Task::new(time,command,schedule_type);

    "Added task"
}

/// remove a task
/// @export
#[extendr]
fn remove_task(task_id: &str) -> &'static str {
    todo!()
}

/// List scheduled tasks
/// @export
#[extendr]
fn list_tasks() -> &'static str {
    todo!()
}

/// Activate a scheduling queue
/// @export
#[extendr]
fn scheduler() -> () {
    todo!()
}

#[derive(Serialize, Deserialize)]
enum Status {
    Unknown,
    Waiting,
    Ready,
    Running,
    Complete,
    Error
}


#[derive(Serialize, Deserialize)]
struct Task {
    id: String,
    time: String,
    command: String,
    schedule_type: String,
    status: Status,
    pid: u32
}

impl Task {
    fn new(time: String, command: String, schedule_type: String) -> Task {
        Task {
            id: "a_task".to_string(),
            time,
            command,
            schedule_type,
            status: Status::Waiting,
            pid: 0
        }
    }

    fn add(&self) -> Result<()> {
        let home_path = match home::home_dir() {
            Some(path) => path,
            None => panic!("Cannot locate home directory!")
        };

        let queue_path = path::Path::new(".schedule/queue/");
        let task_file = home_path.join(queue_path).join("test.json");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(task_file)?;
 

        let serialized = serde_json::to_string(&self)?;
        write!(file,"{}\n", serialized)?;

        Ok(())
    }
}

struct Queue {
    tasks: Vec<Task>
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod schedule;
    fn add_task;
    fn remove_task;
    fn list_tasks;
    fn scheduler;
}
