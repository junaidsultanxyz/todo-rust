use clap::{Args, Parser, Subcommand};
use todo::{clear_list, delete_task, get_tasks, print_tasks, toggle_task_status};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds new task
    Add(AddArgs),

    /// Deletes a task by id
    Delete(DeleteArgs),

    /// Marks task as complete
    Done(DoneArgs),

    /// Marks task as incomplete
    Undone(UndoneArgs),

    /// List tasks
    List(ListArgs),

    Clear,
}

#[derive(Args)]
struct AddArgs {
    name: String,

    #[arg(short = 'd', long = "daily")]
    is_daily: bool,
}

#[derive(Args)]
struct DeleteArgs {
    id: String,
}

#[derive(Args)]
struct DoneArgs {
    id: String,
}

#[derive(Args)]
struct UndoneArgs {
    id: String,
}

#[derive(Args)]
struct ListArgs {
    #[arg(short = 'c', long = "completed")]
    completed: bool,

    #[arg(short = 'a', long = "all")]
    all_items: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(task_args) => {
            let task = if task_args.is_daily {
                todo::add_task(&task_args.name, true)
            } else {
                todo::add_task(&task_args.name, false)
            };

            match task {
                Ok(_) => {
                    println!("[ADD] New task added.");
                }
                Err(err) => {
                    println!("[ERROR] Error while adding new task. {}", err);
                }
            }
        }
        Commands::Delete(task) => {
            match delete_task(&task.id)? {
                Some(task_ok) => {
                    println!("[DELETE] Task {} deleted successfully.", task_ok.id);
                }
                None => {
                    println!("[FAIL] Could not find task with id {}", task.id);
                }
            };
        }
        Commands::List(list_arg) => {
            let tasks = if list_arg.all_items {
                get_tasks(true, false)?
            } else if list_arg.completed {
                get_tasks(false, true)?
            } else {
                get_tasks(false, false)?
            };

            print_tasks(&tasks);
            // println!("[LIST] Listing Tasks");
        }
        Commands::Done(task) => match toggle_task_status(&task.id, true)? {
            true => println!("[COMPLETE] Task {} marked as complete.", task.id),
            false => println!("[FAIL] failed to find task with id {}.", task.id),
        },
        Commands::Undone(task) => match toggle_task_status(&task.id, false)? {
            true => println!("[COMPLETE] Task {} marked as incomplete.", task.id),
            false => println!("[FAIL] failed to find task with id {}.", task.id),
        },
        Commands::Clear => {
            clear_list()?;
            println!("[CLEAR] cleared all tasks")
        }
    }

    Ok(())
}
