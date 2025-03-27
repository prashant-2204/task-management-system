use std::error::Error;

pub fn validate_input(input: &str, field: &str) -> Result<(), String> {
    if input.trim().is_empty() {
        Err(format!("{} cannot be empty", field))
    } else if input.len() > 255 {
        Err(format!("{} too long (max 255 characters)", field))
    } else {
        Ok(())
    }
}

pub fn log_error<E: Error>(error: E) {
    eprintln!("Error occurred: {}", error);
    let mut source = error.source();
    while let Some(e) = source {
        eprintln!("Caused by: {}", e);
        source = e.source();
    }
}