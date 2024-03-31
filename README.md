# Description

_Adding existing time has been disabled._

Taskwarrior hooks to add the UDA elapsed on new tasks and modified tasks, anything with a start and end value. This is useful for tracking time spent on tasks.

It also helps preserve the start and end time of a task, which is removed by Taskwarrior when an end time is added.

This hook is **not** good for tracking time spent on tasks that you toggle on and off and will break any other hooks that are designed to do this.

This hook **is** good for tracking time spent on tasks that you explicilty start and finish and you'd like to keep that information. I use this to track my shifts at work.

~~Elapsed is calculated by `existing_time + (end_date - start_date)`.~~

Elapsed is calculated by `end_date - start_date`.

# Installation

_Need to have `just` installed._

`just install`

This will put `on-add_elapsed` and `on-modify_elapsed` binaries in `~/.task/hooks`.

# Scenarios

Adding/Logging a Task:

* If task has a start and end date: Add elapsed time
* If task is missing a start or end date: Do nothing

Modifying a Task:

* If task has a start and end date: Add elapsed time ~~to existing time~~
* If task is missing a start or end date: Do nothing
