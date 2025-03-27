use crate::database::Database;
use crate::processor::TaskProcessor;
use crate::models::{Task, TaskStatus};

pub struct CommandLineInterface<'a> {
    args: &'a Vec<String>,
}

impl<'a> CommandLineInterface<'a> {
    pub fn new(args: &'a Vec<String>) -> Self {
        CommandLineInterface { args }
    }

    pub fn run(&self, db: &mut Database, processor: &TaskProcessor) -> Result<(), String> {
        if self.args.len() < 2 {
            return self.print_usage();
        }

        match self.args[1].as_str() {
            "add" => self.handle_add(db),
            "list" => self.handle_list(db),
            "complete" => self.handle_complete(db),
            "process" => self.handle_process(db, processor),
            "stats" => self.handle_stats(db),
            _ => self.print_usage(),
        }
    }

    fn print_usage(&self) -> Result<(), String> {
        println!("Usage: {} <command> [args]", self.args[0]);
        println!("Commands:");
        println!("  add <title> <description> - Add a new task");
        println!("  list [status] - List tasks");
        println!("  complete <id> - Mark task as complete");
        println!("  process - Process pending tasks");
        println!("  stats - Show task statistics");
        Ok(())
    }

    fn handle_add(&self, db: &mut Database) -> Result<(), String> {
        if self.args.len() < 4 {
            return Err("Usage: add <title> <description>".to_string());
        }
        let task = Task::new(&self.args[2], &self.args[3]);
        db.add_task(task)?;
        println!("Task added successfully");
        Ok(())
    }

    fn handle_list(&self, db: &Database) -> Result<(), String> {
        let status = if self.args.len() > 2 {
            Some(TaskStatus::from_str(&self.args[2])?)
        } else {
            None
        };
        let tasks = db.list_tasks(status)?;
        for task in tasks {
            println!("{}", task);
        }
        Ok(())
    }

    fn handle_complete(&self, db: &mut Database) -> Result<(), String> {
        if self.args.len() < 3 {
            return Err("Usage: complete <id>".to_string());
        }
        let id: i32 = self.args[2].parse().map_err(|_| "Invalid ID")?;
        db.complete_task(id)?;
        println!("Task {} marked as complete", id);
        Ok(())
    }

    fn handle_process(&self, db: &mut Database, processor: &TaskProcessor) -> Result<(), String> {
        processor.process_tasks(db)?;
        println!("Task processing completed");
        Ok(())
    }

    fn handle_stats(&self, db: &Database) -> Result<(), String> {
        let stats = db.get_statistics()?;
        println!("Task Statistics:");
        println!("Total: {}", stats.total);
        println!("Pending: {}", stats.pending);
        println!("Processing: {}", stats.processing);
        println!("Completed: {}", stats.completed);
        Ok(())
    }
}