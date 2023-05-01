//! Add UDA elapsed time to new tasks.
//!
//! This would typically be for a task that is logged with a given start and end time.

use std::io::{self, BufRead};
use std::io::Write;

//use tasklib::prelude::*;
use tasklib::{self, Task, Duration};

fn main() {
    env_logger::init();

    // Read task JSON from stdin
    let stdin = io::stdin();
    let lines: String = stdin.lock().lines().map(|l| l.unwrap()).collect();

    log::info!("lines: {}", &lines);

    let task: Task = lines.into();

    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return;
    }

    log::info!("task: {:?}", &task);

    let new_task = add_elapsed(task);

    // Write modified task JSON to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let json: String = new_task.into();
    handle.write_all(json.as_bytes()).unwrap();
}

fn add_elapsed(task: Task) -> Task {
    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        return task;
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let duration = end.signed_duration_since(*start);
    let elapsed: Duration = duration.into();

    // UDAs come out just like any other value.
    let mut modified_task = task.clone();
    modified_task.udas_mut().insert("elapsed".into(), elapsed.into());

    modified_task
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add_elapsed() {
        let input = r#"{"id": 0, "urgency": 1.0, "description":"test","entry":"20210101T000000Z","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000"}"#;
        let expected = r#"{"id": 0, "urgency": 1.0, "description":"test","entry":"20210101T000000Z","elapsed":"P2H","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000"}"#;
        assert_eq!(add_elapsed(input.into()).id(), expected.parse::<Task>().unwrap().id());
    }
}
