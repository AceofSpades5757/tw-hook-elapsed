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

    // Read Command Line Arguments
    let _args = CliArguments::from_env().expect("parse command line arguments");

    // Run Hook
    let modified_task = run_on_modify_hook(&pre_task, post_task.as_ref());

    // Write modified task JSON to stdout
    modified_task.to_stdout().unwrap()
}

/// Abstraction to ensure that everything the hook needs to complete is returned.
///
/// Post task is None ({}) if an added task is undone. Return the original task if this happens.
fn run_on_modify_hook(pre_task: &Task, post_task: Option<&Task>) -> Task {
    // An added task was undone.
    if post_task.is_none() {
        log::info!("post_task is empty");
        return pre_task.clone();
    }

    let mut task: Task = post_task.unwrap().clone();

    let mut start = task.start();
    let end = task.end();

    // Start can be on the pre_task if the pre_task has a start and the post_task has an end.
    if start.is_none() {
        start = pre_task.start();
    }

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return post_task.unwrap().clone();
    }

    // Monkey patch start to our task to be modified.
    *task.start_mut() = Some(*start.unwrap());

    log::debug!("task mid: {:?}", &task);
    let modified_task = add_elapsed(task);
    log::info!("post_task: {:?}", &modified_task);
    modified_task
}
