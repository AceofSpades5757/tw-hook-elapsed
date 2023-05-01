//! Add UDA elapsed time to new tasks.
//!
//! This would typically be for a task that is logged with a given start and end time.

use std::io::{self, BufRead};

//use tasklib::prelude::*;
use tasklib::{self, Task, Duration};

fn main() {
    // Read task JSON from stdin
    let stdin = io::stdin();
    let lines: String = stdin.lock().lines().map(|l| l.unwrap()).collect();

    eprintln!("lines: {}", &lines);

    //let task: Task = lines.parse().unwrap();
    //let task: Task = serde_json::from_str(&lines).unwrap();
    let task: Task = lines.into();

    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        eprintln!("start or end is None");
        return;
    }

    eprintln!("task: {:?}", &task);

    let new_task = add_elapsed(task);

    // Write modified task JSON to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    //let json = serde_json::to_string(&new_task).unwrap();
    let json = new_task.to_string();
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
    //let elapsed = end - start;

    //let mut task = task.clone();
    // UDAs come out just like any other value.
    let mut modified_task = task.clone();
    modified_task.set_uda("elapsed", elapsed);
    //let mut task = Task {
        ////elapsed,
        //uda: Some(tasklib::uda::UDA {
            //elapsed: Some(elapsed),
            //..Default::default()
        //}),
        //..task
    //};
    //task.set_elapsed(elapsed);

    task
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"{"description":"test","entry":"2021-01-01T00:00:00Z","modified":"2021-01-01T00:00:00Z","status":"pending","tags":["test"],"uuid":"00000000-0000-0000-0000-000000000000"}"#;
        let expected = r#"{"description":"test","entry":"2021-01-01T00:00:00Z","elapsed":"0:00:00","modified":"2021-01-01T00:00:00Z","status":"pending","tags":["test"],"uuid":"00000000-0000-0000-0000-000000000000"}"#;
        //let task: Task = serde_json::from_str(input).unwrap();
    }
}
