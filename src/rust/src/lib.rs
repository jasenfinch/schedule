use extendr_api::prelude::*;
use std::path;
use rusqlite::{Connection, Result};

/// Add a task
/// @export
#[extendr]
fn add_task(time: String, command: String, schedule_type: String) -> () {
    let queue = Queue::new();
    let _ = queue.add_task(time,command,schedule_type);
    queue.close();
}

/// Remove a task
/// @export
#[extendr]
fn remove_task(task_id: &str) -> () {
    let queue = Queue::new();
    let _ = queue.remove_task(task_id);
    queue.close();
}

/// List scheduled tasks
/// @export
#[extendr]
fn list_tasks() -> () {
    let queue = Queue::new();
    let _ = queue.list_tasks();
    queue.close();
}

/// Activate a scheduling queue
/// @export
#[extendr]
fn scheduler() -> () {
    todo!()
}

enum Status {
    Unknown,
    Waiting,
    Ready,
    Running,
    Complete,
    Error
}


struct Queue {
    conn: rusqlite::Connection
}

impl Queue {
    fn path() -> path::PathBuf {
        let home_path = match home::home_dir() {
            Some(path) => path,
            None => panic!("Cannot locate home directory!")
        };

        let queue_path = home_path.join(path::Path::new(".schedule/queue.db"));

        return queue_path
    }

    fn connection(path: path::PathBuf) -> Result<Connection> {
        Ok(Connection::open(path)?)
    }

    fn close(self) -> () {
        let conn = self.conn;
        let _ = conn.close();
    }

    fn create(conn: &Connection) -> Result<()>{

        conn.execute(
            "create table if not exists queue (
                id integer primary key not null unique,
                time text not null,
                command text not null,
                schedule_type text not null,
                status text not null,
                pid integer
            )",
            (),
        )?;

        Ok(())
    }

    fn new() -> Queue {
        let queue_path = Queue::path();
        let conn = Queue::connection(queue_path).unwrap();

        let _ = Queue::create(&conn);

        Queue {
            conn
        }
    }

    fn add_task(&self, time: String, command: String, schedule_type: String) -> Result<()> {
        let conn = &self.conn;

        conn.execute(
            "INSERT INTO queue (time, command, schedule_type) values (?1, ?2, ?3)",
            [time,command,schedule_type])?;

        Ok(())
    }

    fn remove_task(&self, task_id: &str) -> Result<()> {
        let conn = &self.conn;

        conn.execute(
            "DELETE FROM queue WHERE id = ?1",
            [task_id])?;

        Ok(())
    }

    fn list_tasks(&self) -> () {
        todo!()
    }
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
