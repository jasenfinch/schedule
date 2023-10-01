use std::path;
use chrono;

use crate::queue;
use crate::runner;

#[test]
fn queue_path() {
    let queue_path = queue::Queue::path();
    println!["{}",queue_path.display()];

    assert_eq!(queue_path,home::home_dir()
        .unwrap()
        .join(path::Path::new(".schedule/queue.db")));
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
fn run_task() {
    let pid = runner::run_command("sleep 20 && echo Hello world");

    println!["{:?}",pid];
}

