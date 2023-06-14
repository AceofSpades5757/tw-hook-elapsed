//! on-modify hook that does nothing.

use tasklib::prelude::*;

fn main() {
    env_logger::init();

    // Read task JSON from stdin
    let pre_task: Task = Task::from_stdin().unwrap();
    log::info!("task: {:?}", &pre_task);
    let post_task: Option<Task> = Task::from_stdin().ok();
    log::info!("task: {:?}", &post_task);

    // Run Hook
    let modified_task = run_on_modify_hook(pre_task, post_task);

    // Write modified task JSON to stdout
    match modified_task {
        Some(modified_task) => modified_task.to_stdout().unwrap(),
        None => println!("{{}}"),
    }
}

/// Abstraction to ensure that everything the hook needs to complete is returned.
fn run_on_modify_hook(pre_task: Task, post_task: Option<Task>) -> Option<Task> {
    if post_task.is_none() {
        log::info!("post_task is empty");
        return Some(pre_task);
    }
    let modified_task = post_task.unwrap();
    log::info!("post_task: {:?}", &modified_task);
    Some(modified_task)
}
