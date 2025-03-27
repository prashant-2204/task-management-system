use crate::models::{Task, TaskStatus, TaskStatistics};
use std::fs;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Database {
    tasks: HashMap<i32, Task>,
    next_id: i32,
    path: String,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path).unwrap_or_default();
        let (tasks, next_id) = if content.is_empty() {
            (HashMap::new(), 1)
        } else {
            let db: Database = serde_json::from_str(&content)?;
            (db.tasks, db.next_id)
        };
        Ok(Database {
            tasks,
            next_id,
            path: path.to_string(),
        })
    }

    pub fn add_task(&mut self, mut task: Task) -> Result<i32, String> {
        let id = self.next_id;
        task.id = id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        self.save()?;
        Ok(id)
    }

    pub fn list_tasks(&self, status: Option<TaskStatus>) -> Result<Vec<&Task>, String> {
        let tasks: Vec<&Task> = self.tasks.values()
            .filter(|t| status.map_or(true, |s| t.status == s))
            .collect();
        Ok(tasks)
    }

    pub fn complete_task(&mut self, id: i32) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = TaskStatus::Completed;
            self.save()?;
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn get_task(&mut self, id: i32) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    pub fn get_statistics(&self) -> Result<TaskStatistics, String> {
        let mut stats = TaskStatistics::default();
        for task in self.tasks.values() {
            stats.total += 1;
            match task.status {
                TaskStatus::Pending => stats.pending += 1,
                TaskStatus::Processing => stats.processing += 1,
                TaskStatus::Completed => stats.completed += 1,
            }
        }
        Ok(stats)
    }

    fn save(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| e.to_string())?;
        fs::write(&self.path, content)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}