use crate::{
    save::{TaskListFilter, list_tasks, modify_tasks, save_task},
    task::Task,
};

mod save;
mod task;

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
    let mut tasks = list_tasks(TaskListFilter::All)?;
    if let Some(index) = tasks.iter().position(|task| task.id == id) {
        let removed_item = tasks.remove(index);

        modify_tasks(tasks)?;
        Ok(Some(removed_item))
    } else {
        Ok(None)
    }
}

pub fn toggle_task_status(id: &str, status: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let mut tasks = list_tasks(TaskListFilter::All)?;

    for task in tasks.iter_mut() {
        if task.id == id {
            task.is_completed = status;
            modify_tasks(tasks)?;
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_tasks(all: bool, completed: bool) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if all {
        list_tasks(TaskListFilter::All)
    } else if completed {
        list_tasks(TaskListFilter::Completed)
    } else {
        list_tasks(TaskListFilter::Remaining)
    }
}

pub fn clear_list() -> Result<(), Box<dyn std::error::Error>> {
    let empty_list: Vec<Task> = Vec::new();
    modify_tasks(empty_list)?;
    Ok(())
}

// utility functions

pub fn print_tasks(tasks: &Vec<Task>) {
    let mut basic_tasks = String::new();
    let mut daily_tasks = String::new();

    for task in tasks {
        match task.task_type {
            task::TaskType::Basic => {
                basic_tasks.push_str(
                    &format!("[{0}] {1} | {2}\n", task.is_completed, task.id, task.title)
                        .to_string(),
                );
            }
            task::TaskType::Daily => {
                daily_tasks.push_str(
                    &format!("[{0}] {1} | {2}\n", task.is_completed, task.id, task.title)
                        .to_string(),
                );
            }
        }
    }

    println!("-----Basic tasks-----");
    println!("{basic_tasks}");

    println!("-----Daily tasks-----");
    println!("{daily_tasks}");
}
