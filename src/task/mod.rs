pub enum TaskType {
    Basic,
    Daily,
}

pub struct Task {
    pub id: String,
    pub title: String,
    pub is_completed: bool,
    pub task_type: TaskType,
}

impl Task {
    fn create(title: &str, task_type: TaskType) -> Task {
        Task {
            id: nanoid::nanoid!(),
            title: title.to_string(),
            is_completed: false,
            task_type,
        }
    }

    pub fn basic(title: &str) -> Result<Task, String> {
        Ok(Self::create(title, TaskType::Basic))
    }

    pub fn daily(title: &str) -> Result<Task, String> {
        Ok(Self::create(title, TaskType::Daily))
    }
}
