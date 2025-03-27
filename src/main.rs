mod cli;
mod config;
mod database;
mod models;
mod processor;
mod utils;

use std::env;
use cli::CommandLineInterface;
use config::Config;
use database::Database;
use processor::TaskProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::load()?;
    let mut db = Database::new(&config.database_path)?;
    let processor = TaskProcessor::new(config.worker_threads);
    
    let cli = CommandLineInterface::new(&args);
    cli.run(&mut db, &processor)?;
    
    Ok(())
}