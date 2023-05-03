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
    let post_task: Task = Task::from_stdin().unwrap();
    log::info!("task: {:?}", &post_task);

    // Run Hook
    let modified_task = run_on_modify_hook(pre_task, post_task);

    // Write modified task JSON to stdout
    modified_task.to_stdout().unwrap();
}

/// Abstraction to ensure that everything the hook needs to complete is returned.
fn run_on_modify_hook(_: Task, post_task: Task) -> Task {
    let start = post_task.start();
    let end = post_task.end();

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return post_task;
    }

    let modified_task = add_elapsed(post_task);
    log::info!("post_task: {:?}", &modified_task);
    return modified_task;
}
