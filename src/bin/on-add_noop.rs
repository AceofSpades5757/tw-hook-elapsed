use std::io::{self, Write, BufRead};

use tasklib::Task;

fn main() {

    // Read task JSON from stdin
    let stdin = io::stdin();
    let lines: String = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let task: Task = lines.into();
    dbg!(&task);

    // Write modified task JSON to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(format!("{}\n", task.to_json()).as_bytes()).unwrap();
}
