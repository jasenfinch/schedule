use extendr_api::prelude::*;

#[cfg(test)]
mod tests;
mod queue;

/// Add a task
/// @export
#[extendr]
fn add_task(time: String, command: String, schedule_type: String) -> () {
    let queue = queue::Queue::new();
    queue.add_task(time,command,schedule_type).unwrap();
    queue.close();
}

/// Remove a task
/// @export
#[extendr]
fn remove_task(task_id: &str) -> () {
    let queue = queue::Queue::new();
    queue.remove_task(task_id).unwrap();
    queue.close();
}

/// List scheduled tasks
/// @export
#[extendr]
fn list_tasks() -> () {
    let queue = queue::Queue::new();
    queue.list_tasks().unwrap();
    queue.close();
}

/// Remove a queue
/// @export
#[extendr]
fn remove_queue() -> () {
    queue::Queue::remove();
}

/// Activate a scheduling queue
/// @export
#[extendr]
fn scheduler() -> () {
   todo!(); 
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


