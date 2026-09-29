use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::task::Task;

fn get_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dir = ProjectDirs::from("", "", "todo")
        .ok_or("[SYS-ERROR] Failed to determine system directories")?;

    let data_dir = project_dir.data_dir();
    let file_path = data_dir.join("tasks.json");

    Ok(file_path)
}

pub fn save_task(task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_path()?;

    let mut tasks = load_tasks()?;
    tasks.push(task.clone());

    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks)?;

    Ok(())
}

pub fn modify_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_path()?;

    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks)?;

    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = get_path()?;
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, [])?;
        return Ok(Vec::new());
    }

    let file = File::open(&path)?;

    let reader = BufReader::new(file);
    let tasks: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(res) => res,
        Err(_) => {
            return Ok(Vec::new());
        }
    };

    Ok(tasks)
}
