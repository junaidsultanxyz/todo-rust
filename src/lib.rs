use crate::{
    save::{TaskListFilter, list_tasks, save_task},
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
    println!("[ADD] Task add successful");
    Ok(())
}

// pub fn delete_task(id: &str) -> Result<String, String> {
//     todo!("find task");
//     todo!("remove task from file");
//     todo!("delete task by id")
// }

pub fn get_tasks(all: bool, completed: bool) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if all {
        list_tasks(TaskListFilter::All)
    } else if completed {
        list_tasks(TaskListFilter::Completed)
    } else {
        list_tasks(TaskListFilter::Remaining)
    }
}
