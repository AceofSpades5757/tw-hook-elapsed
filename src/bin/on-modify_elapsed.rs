//! Add UDA elapsed time to modified tasks.
//!
//! Adds time to existing time if an existing time is present.

use tasklib::prelude::*;

use elapsed::add_elapsed;

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
///
/// Post task is None ({}) if the task is deleted.
///
/// Return None ({}) if the task is deleted.
fn run_on_modify_hook(_: Task, post_task: Option<Task>) -> Option<Task> {
    if post_task.is_none() {
        log::info!("post_task is None");
        return post_task;
    }
    let post_task = post_task.unwrap();

    let start = post_task.start();
    let end = post_task.end();

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return Some(post_task);
    }

    let modified_task = add_elapsed(post_task);
    log::info!("post_task: {:?}", &modified_task);
    Some(modified_task)
}
