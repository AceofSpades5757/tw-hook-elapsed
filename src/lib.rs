use tasklib::prelude::*;

pub fn add_elapsed(task: Task) -> Task {
    let start = task.start();
    let end = task.end();

    if start.is_none() || end.is_none() {
        return task;
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let existing_elapsed = task.udas().get("elapsed").unwrap_or(&UdaValue::Duration(Duration::days(0))).clone();
    let existing_elapsed: Duration = existing_elapsed.try_into().unwrap();

    let duration = end.signed_duration_since(*start);
    let elapsed: Duration = duration.into();

    let total_elapsed = existing_elapsed + elapsed;

    // UDAs come out just like any other value.
    let mut modified_task = task.clone();
    modified_task.udas_mut().insert("elapsed".into(), total_elapsed.into());

    modified_task
}
