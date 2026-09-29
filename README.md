# Todo Rust CLI

This is a CLI based todo app. It can be used in other Rust projects as well as in CLI.

---

## How it works

Saves all the tasks in `tasks.json` file located at the local data of your OS. All the file related functionality is done using `serde` and `serde_json`. File related functions are located in `save.rs` file. The CLI is made using `clap` library.

For tasks which are marked as daily, the completion status resets everyday at 00:00, local time.

---

## Usage Commands

```
Usage: todo <COMMAND>

Commands:
  add     Adds new task
  delete  Deletes a task by id
  done    Marks task as complete
  undone  Marks task as incomplete
  list    List tasks
  clear   Deletes all tasks
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---

# License

Open-source under the [MIT License](https://github.com/junaidsultanxyz/todo-rust/blob/main/LICENSE).
