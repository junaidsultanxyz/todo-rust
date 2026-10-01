use crate::{
    save::{load_tasks, modify_tasks, save_task},
    task::{Task, TaskType},
};

mod save;
pub mod task;

pub fn add_task(title: &str, is_daily: bool) -> Result<(), Box<dyn std::error::Error>> {
    let new_task = if is_daily {
        Task::daily(title)
    } else {
        Task::basic(title)
    };

    save_task(new_task)?;
    Ok(())
}

pub fn delete_task(id: &str) -> Result<Option<Task>, Box<dyn std::error::Error>> {
    let mut tasks = load_tasks()?;
    if let Some(index) = tasks.iter().position(|task| task.id == id) {
        let removed_item = tasks.remove(index);

        modify_tasks(tasks)?;
        Ok(Some(removed_item))
    } else {
        Ok(None)
    }
}

pub fn toggle_task_status(id: &str, status: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let mut tasks = load_tasks()?;

    for task in tasks.iter_mut() {
        if task.id == id {
            if status {
                task.complete();
            } else {
                task.uncomplete();
            }

            modify_tasks(tasks)?;
            return Ok(true);
        }
    }

    Ok(false)
}

pub enum TaskListFilter {
    All,
    Completed,
    Remaining,
}
pub fn get_tasks(
    filter: TaskListFilter,
    task_type: Option<TaskType>,
) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let tasks = load_tasks()?;

    let tasks = match filter {
        TaskListFilter::All => tasks,
        TaskListFilter::Completed => tasks
            .into_iter()
            .filter(|task| task.is_completed())
            .collect(),
        TaskListFilter::Remaining => tasks
            .into_iter()
            .filter(|task| !task.is_completed())
            .collect(),
    };

    let tasks = match task_type {
        Some(task_type) => tasks
            .into_iter()
            .filter(|task| task.task_type == task_type)
            .collect(),
        None => tasks,
    };

    Ok(tasks)
}

pub fn clear_list() -> Result<(), Box<dyn std::error::Error>> {
    let empty_list: Vec<Task> = Vec::new();
    modify_tasks(empty_list)?;
    Ok(())
}

// utility functions

pub fn print_tasks(tasks: &[Task]) {
    let (daily_tasks, basic_tasks): (Vec<&Task>, Vec<&Task>) = tasks
        .iter()
        .partition(|t| matches!(t.task_type, TaskType::Daily));

    print_section("Daily Tasks", &daily_tasks);
    print_section("Basic Tasks", &basic_tasks);
}

fn print_section(header: &str, tasks: &[&Task]) {
    println!("─── {header} ───");

    if tasks.is_empty() {
        println!("  (none)\n");
        return;
    }

    for task in tasks {
        let status = if task.is_completed() { "X" } else { " " };
        // Aligns ID to at least 3 digits (e.g., "  1", " 10", "100")
        println!("  [{status}] {:<3} │ {}", task.id, task.title);
    }

    println!();
}
