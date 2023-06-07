//! Add UDA elapsed time to new tasks.
//!
//! This would typically be for a task that is logged with a given start and end time.
//!
//! WARNING: TaskWarrior will _not_ include `end` for newly added tasks. Although we are prepared
//! for this eventuality, it is simply not supported.

use tasklib::prelude::*;

use elapsed::add_elapsed;

fn main() {
    env_logger::init();

    // Read task JSON from stdin
    let task: Task = Task::from_stdin().unwrap();

    // Run Hook
    let modified_task = run_on_add_hook(task);

    // Write modified task JSON to stdout
    modified_task.to_stdout().unwrap();
}

/// Abstraction to ensure that everything the hook needs to complete is returned.
fn run_on_add_hook(task: Task) -> Task {
    log::info!("task: {:?}", &task);

    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return task;
    }

    let modified_task = add_elapsed(task);
    log::info!("modified task: {:?}", &modified_task);
    modified_task
}
