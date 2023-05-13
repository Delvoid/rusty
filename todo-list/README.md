# Todo CLI

This is a simple command-line application written in Rust that helps manage a list of todo tasks.

## Features

- Add a new task
- Mark a task as done or undone
- View all tasks
- Delete a task
- Quit the application

## How to Use

You can interact with the application using the following commands:

- To add a new task, type `Add "your task here"`. For example, `Add "Learn Rust"`.
- To mark a task as done or undone, type `Done [index]`. The index is the position of the task in the list, starting from 0. For example, `Done 1`.
- To view all tasks, simply type `View`.
- To delete a task, type `Delete [index]`. For example, `Delete 1`.
- To quit the application, simply type `Quit`.

## Building the Application

To build this application, ensure you have Rust installed on your system. Navigate to the project directory and use the following command:

```bash
cargo build --release
```

This will create a binary in ./target/release/.

To make the binary executable from any location on your system, move it to a location in your system's PATH. For example, on a Unix-like system like MacOS or Linux, you might move it to /usr/local/bin:

```bash
mv ./target/release/todo-list /usr/local/bin/
```

Now you can run the todo command from any location in your terminal.

## Error Handling
The application handles errors such as loading and saving todos. If an error occurs, it will print a warning message but will not crash the application.

## Persistence
The application stores the todos in a file named "todos.txt", allowing the data to persist between sessions.