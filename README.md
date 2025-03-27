# Task Management System

A robust command-line task management system built in Rust, designed to efficiently manage tasks with multi-threaded processing and persistent storage.

## Features

- **Command-Line Interface**: Add, list, complete, and process tasks with simple commands.
- **Multi-threaded Processing**: Handles task processing in parallel using a configurable worker pool.
- **Persistent Storage**: Stores tasks in a JSON-based database with automatic saving.
- **Task Statistics**: Provides detailed statistics on task status (total, pending, processing, completed).
- **Configurable**: Customizable via a JSON configuration file for database path and worker threads.
- **Error Handling**: Robust error management with input validation and logging.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/prashant-2204/task-management-system.git
    cd task-management-system
    ```

2. **Build the Project**:
    ```bash
    cargo build --release
    ```

3. **(Optional) Configure**: Create a `config.json` file in the root directory (defaults will be used if not provided):
    ```json
    {
        "database_path": "tasks.db",
        "worker_threads": 4
    }
    ```

4. **Run the Application**:
    ```bash
    cargo run --release
    ```

## Usage

The application supports the following commands:

- **Add a Task**:
    ```bash
    cargo run --release -- add "Task Title" "Task Description"
    ```
    Adds a new task with the given title and description.

- **List Tasks**:
    ```bash
    cargo run --release -- list [status]
    ```
    Lists all tasks or filters by status (e.g., pending, processing, completed).
    Example:
    ```bash
    cargo run --release -- list pending
    ```

- **Complete a Task**:
    ```bash
    cargo run --release -- complete <task_id>
    ```
    Marks a task as completed by its ID.
    Example:
    ```bash
    cargo run --release -- complete 1
    ```

- **Process Tasks**:
    ```bash
    cargo run --release -- process
    ```
    Processes all pending tasks using the worker pool (simulates processing with a 1-second delay).

- **View Statistics**:
    ```bash
    cargo run --release -- stats
    ```
    Displays task statistics (total, pending, processing, completed).

### Example Workflow

1. Add tasks:
    ```bash
    cargo run --release -- add "Write Report" "Draft quarterly report"
    cargo run --release -- add "Code Review" "Review team PRs"
    ```

2. List all tasks:
    ```bash
    cargo run --release -- list
    ```

3. Process pending tasks:
    ```bash
    cargo run --release -- process
    ```

4. Complete a task:
    ```bash
    cargo run --release -- complete 1
    ```

5. View stats:
    ```bash
    cargo run --release -- stats
    ```

## Configuration Options

Edit `config.json` to customize the system:

| Field          | Type   | Default       | Description                         |
|----------------|--------|---------------|-------------------------------------|
| `database_path`| String | `"tasks.db"`  | Path to the task database file      |
| `worker_threads`| usize  | `4`           | Number of worker threads            |

## Project Structure

- `src/main.rs`: Application entry point
- `src/cli.rs`: Command-line interface implementation
- `src/config.rs`: Configuration loading and management
- `src/database.rs`: Task storage and retrieval
- `src/models.rs`: Task data structures and enums
- `src/processor.rs`: Multi-threaded task processing
- `src/utils.rs`: Utility functions for validation and logging

## Technical Details

- **Rust**: Built with Rust for performance and safety.
- **Tokio**: Not used directly, but multi-threading is implemented with standard library threads.
- **Serde**: Used for JSON serialization/deserialization of tasks and configuration.
- **Chrono**: Handles timestamp generation for task creation.

## Performance

- Efficient task processing with multi-threading
- Minimal memory footprint
- Fast JSON-based persistence

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am "Add your feature"`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Create a Pull Request

## Development Setup

1. **Install Rust**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Run tests**:
    ```bash
    cargo test
    ```

3. **Format code**:
    ```bash
    cargo fmt
    ```

4. **Check linting**:
    ```bash
    cargo clippy
    ```
