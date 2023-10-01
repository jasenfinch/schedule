use std::path;
use chrono;

use crate::queue;

#[test]
fn queue_path() {
    let queue_path = queue::Queue::path();
    println!["{}",queue_path.display()];

    assert_eq!(queue_path,home::home_dir()
        .unwrap()
        .join(path::Path::new(".schedule/queue.db")));
}

#[test]
fn create_queue(){
    queue::Queue::new();
    queue::Queue::remove();
}

#[test]
fn add_task() {
    queue::Queue::remove();
    let queue = queue::Queue::new();
    queue.add_task(
        chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "echo Hello World".to_string(),
        vec![queue::Type::Load,queue::Type::Position]
    )
    .unwrap();

    println!["{:?}",queue.list_tasks().unwrap()];

    queue::Queue::remove();
}

#[test]
fn type_test() {
    let type_vec: Vec<queue::Type> = "Load Position"
        .split(" ")
        .map(|x| match x {
            "Time" => queue::Type::Time,
            "Position" => queue::Type::Position,
            "Load" => queue::Type::Load,
            _ => panic!("Task scheduling type not recognised")
        })
        .collect();
    println!["{:?}",type_vec];
}

