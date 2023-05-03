use tasklib::Task;

fn main() {
    env_logger::init();

    // Read task JSON from stdin
    let task: Task = Task::from_stdin().unwrap();

    log::debug!("{:?}", &task);

    // Write modified task JSON to stdout
    task.to_stdout().unwrap();
}
