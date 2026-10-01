use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

const CLI_ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

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
    pub task_type: TaskType,
    pub last_completed_at: Option<DateTime<Utc>>,
}

impl Task {
    fn create(title: &str, task_type: TaskType) -> Task {
        Task {
            id: nanoid::nanoid!(6, &CLI_ALPHABET),
            title: title.to_string(),
            last_completed_at: None,
            task_type,
        }
    }

    pub fn basic(title: &str) -> Task {
        Self::create(title, TaskType::Basic)
    }

    pub fn daily(title: &str) -> Task {
        Self::create(title, TaskType::Daily)
    }

    pub fn is_completed(&self) -> bool {
        match self.last_completed_at {
            None => false,
            Some(completed_at) => match self.task_type {
                TaskType::Daily => {
                    let completed_local_date = completed_at.with_timezone(&Local).date_naive();
                    let today_local = Local::now().date_naive();

                    eprintln!(
                        "DEBUG: completed_local = {completed_local_date}, today_local = {today_local}"
                    );
                    completed_local_date >= today_local
                }
                _ => true,
            },
        }
    }

    pub fn complete(&mut self) {
        self.last_completed_at = Some(Utc::now());
    }

    pub fn uncomplete(&mut self) {
        self.last_completed_at = None;
    }

    pub fn json(&self) -> Option<String> {
        let json = to_string(self);
        if json.is_ok() { json.ok() } else { None }
    }
}
