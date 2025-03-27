use crate::database::Database;
use crate::models::{Task, TaskStatus};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

pub struct TaskProcessor {
    sender: Sender<i32>,
    receiver: Arc<Mutex<Receiver<i32>>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl TaskProcessor {
    pub fn new(num_threads: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut handles = Vec::new();

        for _ in 0..num_threads {
            let rx_clone = Arc::clone(&rx);
            let handle = thread::spawn(move || {
                while let Ok(task_id) = rx_clone.lock().unwrap().recv() {
                    // Simulate task processing
                    thread::sleep(std::time::Duration::from_secs(1));
                    println!("Processed task {}", task_id);
                }
            });
            handles.push(handle);
        }

        TaskProcessor {
            sender: tx,
            receiver: rx,
            handles,
        }
    }

    pub fn process_tasks(&self, db: &mut Database) -> Result<(), String> {
        let pending_tasks: Vec<i32> = db.list_tasks(Some(TaskStatus::Pending))?
            .into_iter()
            .map(|t| t.id)
            .collect();

        for task_id in pending_tasks {
            if let Some(task) = db.get_task(task_id) {
                task.status = TaskStatus::Processing;
            }
            self.sender.send(task_id).map_err(|e| e.to_string())?;
        }

        db.save()?;
        Ok(())
    }
}

impl Drop for TaskProcessor {
    fn drop(&mut self) {
        drop(self.sender.clone());
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
    }
}