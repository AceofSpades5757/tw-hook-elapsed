//! Add UDA elapsed time to new tasks.
//!
//! This would typically be for a task that is logged with a given start and end time.

use tasklib::prelude::*;

fn main() {
    env_logger::init();

    // Read task JSON from stdin
    let task: Task = Task::from_stdin().unwrap();
    log::info!("task: {:?}", &task);

    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        log::info!("start or end is None");
        return;
    }

    let modified_task = add_elapsed(task);
    log::info!("task: {:?}", &modified_task);

    // Write modified task JSON to stdout
    modified_task.to_stdout().unwrap();
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
    fn start_and_end() {
        let input = r#"{"description":"test","entry":"20210101T000000Z","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000"}"#;
        let expected = r#"{"description":"test","entry":"20210101T000000Z","elapsed":"P2H","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000"}"#;
        assert_eq!(
            add_elapsed(input.into()).id(),
            expected.parse::<Task>().unwrap().id(),
        );
    }
    #[test]
    fn no_start_no_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending"}"#;
        let expexted = input;
        assert_eq!(
            add_elapsed(input.into()).id(),
            expexted.parse::<Task>().unwrap().id(),
        );
    }
    #[test]
    fn no_start() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending", "end":"20210101T020000Z"}"#;
        let expexted = input;
        assert_eq!(
            add_elapsed(input.into()).id(),
            expexted.parse::<Task>().unwrap().id(),
        );
    }
    #[test]
    fn no_end() {
        let input = r#"{"uuid":"00000000000000000000000000000000", "description":"test", "entry":"20210101T000000Z", "modified":"20210101T000000Z", "status":"pending", "start":"20210101T020000Z"}"#;
        let expexted = input;
        assert_eq!(
            add_elapsed(input.into()).id(),
            expexted.parse::<Task>().unwrap().id(),
        );
    }
}
