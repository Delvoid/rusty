use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, stdin, BufReader, BufWriter, Write};
use std::str::FromStr;

#[derive(Debug)]
struct Todo {
    name: String,
    completed: bool,
}

impl Todo {
    fn new(name: String) -> Self {
        Self {
            name,
            completed: false,
        }
    }
}

enum Command {
    Add(String),
    View,
    Delete(usize),
    Complete(usize),
    Quit,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts[0] {
            "view" => Ok(Command::View),
            "add" => {
                if parts.len() < 2 {
                    Err("Not enough arguments for 'add'")
                } else {
                    Ok(Command::Add(parts[1..].join(" ")))
                }
            }
            "done" => {
                if parts.len() < 2 {
                    Err("Not enough arguments for 'done'")
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(index) => Ok(Command::Complete(index)),
                        Err(_) => Err("Failed to parse index for 'done'"),
                    }
                }
            }
            "delete" => {
                if parts.len() < 2 {
                    Err("Not enough arguments for 'delete'")
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(index) => Ok(Command::Delete(index)),
                        Err(_) => Err("Failed to parse index for 'delete'"),
                    }
                }
            }
            "quit" => Ok(Command::Quit),
            _ => Err("Unknown command"),
        }
    }
}

fn main() -> io::Result<()> {
    let file_path = "todos.txt";
    let mut todos = load_todos(file_path).unwrap_or_else(|err| {
        eprintln!("Error loading todos: {}", err);
        vec![]
    });

    loop {
        println!("What would you like to do?");
        println!("- Add \"todo item\"");
        println!("- Done [index]");
        println!("- Delete [index]");
        println!("- View");
        println!("- Quit");

        let choice = get_input();

        match choice.parse::<Command>() {
            Ok(command) => match command {
                Command::Add(name) => add_task(&mut todos, name, file_path),
                Command::View => view_tasks(&todos),
                Command::Delete(index) => delete_task(&mut todos, index, file_path),
                Command::Complete(index) => complete_task(&mut todos, index, file_path),
                Command::Quit => break,
            },
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_task(todos: &mut Vec<Todo>, name: String, file_path: &str) {
    todos.push(Todo::new(name));
    if let Err(err) = save_todos(todos, file_path) {
        eprintln!("Warning: failed to save todos: {}", err);
    }
}

fn view_tasks(todos: &[Todo]) {
    for (index, todo) in todos.iter().enumerate() {
        println!(
            "[{}] - {} [{}]",
            index,
            todo.name,
            if todo.completed { "X" } else { " " }
        );
    }
}

fn delete_task(todos: &mut Vec<Todo>, index: usize, file_path: &str) {
    if index < todos.len() {
        todos.remove(index);
        if let Err(err) = save_todos(todos, file_path) {
            eprintln!("Warning: failed to save todos: {}", err);
        }
    }
}

fn complete_task(todos: &mut [Todo], index: usize, file_path: &str) {
    if let Some(todo) = todos.get_mut(index) {
        todo.completed = !todo.completed;
        if let Err(err) = save_todos(todos, file_path) {
            eprintln!("Warning: failed to save todos: {}", err);
        }
    }
}

fn load_todos(file_path: &str) -> io::Result<Vec<Todo>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let reader = BufReader::new(file);

    let mut todos = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        let todo = Todo {
            name: parts[0].to_string(),
            completed: parts[1].parse().unwrap(),
        };

        todos.push(todo);
    }

    Ok(todos)
}

fn save_todos(todos: &[Todo], file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    for todo in todos {
        writeln!(writer, "{},{}", todo.name, todo.completed)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_new() {
        let todo = Todo::new("Test".to_string());
        assert_eq!(todo.name, "Test");
        assert_eq!(todo.completed, false);
    }

    #[test]
    fn test_command_from_str() {
        let add_command = "add Test".parse::<Command>();
        assert!(matches!(add_command, Ok(Command::Add(_))));

        let view_command = "view".parse::<Command>();
        assert!(matches!(view_command, Ok(Command::View)));

        let delete_command = "delete 1".parse::<Command>();
        assert!(matches!(delete_command, Ok(Command::Delete(_))));

        let complete_command = "done 1".parse::<Command>();
        assert!(matches!(complete_command, Ok(Command::Complete(_))));

        let quit_command = "quit".parse::<Command>();
        assert!(matches!(quit_command, Ok(Command::Quit)));

        let unknown_command = "unknown".parse::<Command>();
        assert!(matches!(unknown_command, Err(_)));
    }

    // This is an example of a test that requires file I/O. Note that it can potentially
    // interfere with the actual todos file if run alongside the application.
    #[test]
    fn test_load_save_todos() {
        let todos = vec![
            Todo::new("Test 1".to_string()),
            Todo::new("Test 2".to_string()),
        ];
        let file_path = "test_todos.txt";
        assert!(save_todos(&todos, file_path).is_ok());

        let loaded_todos = load_todos(file_path);
        assert!(loaded_todos.is_ok());

        let loaded_todos = loaded_todos.unwrap();
        assert_eq!(todos.len(), loaded_todos.len());
        for (expected, actual) in todos.iter().zip(loaded_todos.iter()) {
            assert_eq!(expected.name, actual.name);
            assert_eq!(expected.completed, actual.completed);
        }

        // Cleanup after test
        std::fs::remove_file(file_path).unwrap_or_else(|err| {
            panic!("Failed to cleanup test file: {}", err);
        });
    }
}
