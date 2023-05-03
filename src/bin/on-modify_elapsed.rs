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

    let start = post_task.start();
    let end = post_task.end();

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return;
    }

    let modified_task = add_elapsed(post_task);
    log::info!("post_task: {:?}", &modified_task);

    // Write modified task JSON to stdout
    modified_task.to_stdout().unwrap();
}
