use clap::{Args, Parser, Subcommand};

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

#[derive(Args)]
struct ListArgs {
    #[arg(short = 'c', long = "completed")]
    completed: bool,

    #[arg(short = 'a', long = "all")]
    all_items: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(task) => {
            println!(
                "[ADD] added task '{}' and is_daily is '{}'",
                task.name, task.is_daily
            );
        }
        Commands::Delete(task) => {
            println!("[DELETE] deleted task with id '{}'", task.id)
        }
        Commands::List(list_arg) => {
            println!(
                "[LIST] listing {1} the {0} items",
                {
                    if list_arg.completed {
                        "completed"
                    } else {
                        "remaining"
                    }
                },
                { if list_arg.all_items { "all" } else { "only" } }
            );
        }
    }
}
