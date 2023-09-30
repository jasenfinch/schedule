use std::path;

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
        "test".to_string(),
        "test".to_string(),
        "Load".to_string()
    )
    .unwrap();

    println!["{:?}",queue.list_tasks().unwrap()];

    queue::Queue::remove();
}
