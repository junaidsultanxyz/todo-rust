use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Basic,
    Daily,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn basic(title: &str) -> Task {
        Self::create(title, TaskType::Basic)
    }

    pub fn daily(title: &str) -> Task {
        Self::create(title, TaskType::Daily)
    }

    pub fn complete(&mut self) {
        self.is_completed = true;
    }

    pub fn json(&self) -> Option<String> {
        let json = to_string(self);
        if json.is_ok() { json.ok() } else { None }
    }
}
