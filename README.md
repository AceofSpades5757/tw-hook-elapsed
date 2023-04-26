# Description

Taskwarrior hooks to add the UDA elapsed on new tasks and modified tasks.

Elapsed is calculated by `existing_time + (end_date - start_date)`.

# Installation

`cargo install --path .`

This will put `on-add_elapsed` and `on-modify_elapsed` binaries in `~/.task/hooks`.

# Scenarios

_Note: When an end time is given to a task, Taskwarrior removes the start time._

Adding/Logging a Task:

* If task has a start and end date: Add elapsed time
* If task is missing a start or end date: Do nothing

Modifying a Task:

* If task has a start and end date: Add elapsed time to existing time, 0 if nothing is there