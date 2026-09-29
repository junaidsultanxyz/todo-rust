use clap::{Args, Parser, Subcommand};
use todo::{get_tasks, print_tasks};

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

    /// Toggles task complete status
    Complete,

    /// List tasks
    List(ListArgs),
}

#[derive(Args)]
struct AddArgs {
    name: String,

    #[arg(short = 'd', long = "daily")]
    is_daily: bool,
}

#[derive(Args)]
struct DeleteArgs {
    id: u32,
}

// #[derive(Args)]
// struct CompleteArgs {}

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
            println!("[DELETE] deleted task with id '{}'", task.id)
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
        Commands::Complete => {
            todo!()
        }
    }

    Ok(())
} // [X] <id> | Task
