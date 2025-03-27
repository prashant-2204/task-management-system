use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
}

impl TaskStatus {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(TaskStatus::Pending),
            "processing" => Ok(TaskStatus::Processing),
            "completed" => Ok(TaskStatus::Completed),
            _ => Err("Invalid status".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: String,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Self {
        Task {
            id: 0,
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {} | {} | {} | Status: {:?} | Created: {}",
            self.id, self.title, self.description, self.status, self.created_at
        )
    }
}

#[derive(Default)]
pub struct TaskStatistics {
    pub total: u32,
    pub pending: u32,
    pub processing: u32,
    pub completed: u32,
}