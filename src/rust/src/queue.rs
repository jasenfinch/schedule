use std::path;
use std::fs;
use rusqlite::{Connection, Result};

#[derive(Debug)]
enum Status {
    Unknown,
    Waiting,
    Ready,
    Running,
    Complete,
    Error
}

#[derive(Debug)]
enum Type {
    Time,
    Position,
    Load
}

#[derive(Debug)]
pub struct Task {
    id: u32,
    time: String, 
    command: String, 
    schedule_type: Type,
    status: Status,
    pid: Option<u32>
}


pub struct Queue {
    conn: rusqlite::Connection
}

impl Queue {
    pub fn path() -> path::PathBuf {
        let home_path = match home::home_dir() {
            Some(path) => path,
            None => panic!("Cannot locate home directory!")
        };

        let queue_path = home_path.join(path::Path::new(".schedule/queue.db"));

        return queue_path
    }

    fn connection(path: path::PathBuf) -> Result<Connection> {
        if !path.exists() {
            fs::create_dir_all(&path.parent().unwrap()).unwrap();
            fs::File::create(&path).unwrap(); 
        }

        Ok(Connection::open(path)?)
    }

    pub fn close(self) -> () {
        let conn = self.conn;
        conn.close()
            .unwrap();
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

    pub fn new() -> Queue {
        let queue_path = Queue::path();
        let conn = Queue::connection(queue_path).unwrap();

        Queue::create(&conn).unwrap();

        Queue {
            conn
        }
    }

    pub fn remove() -> () {
        let queue_path = Queue::path();
        let queue = Queue::new();
        queue.close();
        
        if queue_path.exists() {
            fs::remove_file(queue_path).unwrap();
        }
    }

    pub fn add_task(&self, time: String, command: String, schedule_type: String) -> Result<()> {
        let conn = &self.conn;

        conn.execute(
            "INSERT INTO queue (time, command, schedule_type, status) values (?1, ?2, ?3, ?4)",
            [time,command,schedule_type,"Waiting".to_string()])?;

        Ok(())
    }

    pub fn remove_task(&self, task_id: &str) -> Result<()> {
        let conn = &self.conn;

        conn.execute(
            "DELETE FROM queue WHERE id = ?1",
            [task_id])?;

        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let conn = &self.conn;

        let mut query = conn.prepare("SELECT * FROM queue")?;

        let tasks = query.query_map((), |row| {
            Ok(Task {
                id: row.get(0)?,
                time: row.get(1)?,
                command: row.get(2)?,
                schedule_type: match row.get::<usize,String>(3)?.as_str() {
                    "Time" => Type::Time,
                    "Position" => Type::Position,
                    "Load" => Type::Load,
                    _ => panic!("Invalid schedule type")
                },
                status: match row.get::<usize,String>(4)?.as_str() {
                    "Unknown" => Status::Unknown,
                    "Waiting" => Status::Waiting,
                    "Ready" => Status::Ready,
                    "Running" => Status::Running,
                    "Complete" => Status::Complete,
                    "Error" => Status::Error,
                    _ => panic!("Invalid task status")
                        
                },
                pid: row.get(5)?
            })
        })?.collect();
        
        return tasks
    }
}
